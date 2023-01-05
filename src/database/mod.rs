pub mod models;

use crate::database::models::{Product, User};
use crate::schema::products as products_schema;
use crate::schema::products::dsl::products;
use crate::schema::users as users_schema;
use crate::schema::users::dsl::users;
use crate::{schema, CONFIGURATION};
use diesel::{Connection, ExpressionMethods, Insertable, PgConnection, QueryDsl, RunQueryDsl};
use rocket_sync_db_pools::database;

/// A wrapper that can serve as a request guard of any rocket route.
#[database("webshop")]
pub struct WebshopDatabase(diesel::PgConnection);

/// Run all the pending migrations for the database specified by environment variable
/// WEBSHOP_DATABASE_URL.
pub fn run_pending_migrations() -> Result<(), Box<dyn std::error::Error>> {
    let connection: PgConnection =
        diesel::connection::Connection::establish(&CONFIGURATION.get().unwrap().database_url)
            .unwrap();
    diesel_migrations::run_pending_migrations(&connection)
        .map_err(|_| Box::from("Can't run migrations."))
}

/// Any error that can arise from interaction with the database.
#[non_exhaustive]
pub enum DatabaseError {
    Query,
}

/// Represents a new product that wants to be inserted into the database.
#[derive(Insertable)]
#[table_name = "products_schema"]
pub struct NewProduct {
    pub name: String,
    pub description: String,
    pub image_uri: String,
    pub price: f32,
}

/// Represents a new user that wants to be inserted into the database.
#[derive(Insertable)]
#[table_name = "users_schema"]
pub struct NewUser {
    pub first_name: String,
    pub surname: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

/// Represents an update to a single product in the database.
pub struct UpdateProduct {
    pub product_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f32>,
    pub image_uri: Option<String>,
}

pub async fn update_product(
    database: &WebshopDatabase,
    update: UpdateProduct,
) -> Result<Product, DatabaseError> {
    database
        .run(move |c| {
            c.transaction::<_, diesel::result::Error, _>(|| {
                if let Some(name) = update.name {
                    diesel::update(products)
                        .filter(products_schema::product_id.eq(update.product_id))
                        .set(products_schema::name.eq(name))
                        .execute(c)?;
                }
                if let Some(description) = update.description {
                    diesel::update(products)
                        .filter(products_schema::product_id.eq(update.product_id))
                        .set(products_schema::description.eq(description))
                        .execute(c)?;
                }
                if let Some(price) = update.price {
                    diesel::update(products)
                        .filter(products_schema::product_id.eq(update.product_id))
                        .set(products_schema::price.eq(price))
                        .execute(c)?;
                }
                if let Some(image_uri) = update.image_uri {
                    diesel::update(products)
                        .filter(products_schema::product_id.eq(update.product_id))
                        .set(products_schema::image_uri.eq(image_uri))
                        .execute(c)?;
                }
                Ok(())
            })
            .unwrap();
            products
                .filter(products_schema::product_id.eq(update.product_id))
                .first::<Product>(c)
                .map_err(|_| DatabaseError::Query)
        })
        .await
}

pub async fn create_product(
    database: &WebshopDatabase,
    new_product: NewProduct,
) -> Result<Product, DatabaseError> {
    database
        .run(|c| {
            diesel::insert_into(products)
                .values(new_product)
                .get_result(c)
                .map_err(|_| DatabaseError::Query)
        })
        .await
}

pub async fn user_with_email(
    database: &WebshopDatabase,
    email: &str,
) -> Result<User, DatabaseError> {
    let email = email.to_owned();
    database
        .run(move |c| {
            users
                .filter(schema::users::email.eq(email))
                .first::<User>(c)
                .map_err(|_| DatabaseError::Query)
        })
        .await
}

pub async fn delete_user(database: &WebshopDatabase, user_id: i64) -> Result<(), DatabaseError> {
    database
        .run(move |c| {
            diesel::update(users)
                .filter(schema::users::user_id.eq(user_id))
                .set(schema::users::is_active.eq(false))
                .execute(c)
                .map_err(|_| DatabaseError::Query)
                .map(|_| ())
        })
        .await
}

pub async fn create_user(
    database: &WebshopDatabase,
    new_user: NewUser,
) -> Result<User, DatabaseError> {
    database
        .run(move |c| {
            diesel::insert_into(users)
                .values(new_user)
                .get_result::<User>(c)
                .map_err(|_| DatabaseError::Query)
        })
        .await
}

pub async fn fetch_users(database: &WebshopDatabase) -> Result<Vec<User>, DatabaseError> {
    database
        .run(|c| {
            users
                .get_results::<User>(c)
                .map_err(|_| DatabaseError::Query)
        })
        .await
}
