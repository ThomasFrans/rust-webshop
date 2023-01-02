pub mod models;

use crate::database::models::{Product, User};
use crate::schema;
use crate::schema::products as products_schema;
use crate::schema::products::dsl::products;
use crate::schema::users as users_schema;
use crate::schema::users::dsl::users;
use diesel::data_types::PgNumeric;
use diesel::result::Error;
use diesel::{ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, Table};
use rocket::http::hyper::body::HttpBody;
use rocket_sync_db_pools::database;

/// A wrapper that can serve as a request guard of any rocket route.
#[database("webshop")]
pub struct WebshopDatabase(diesel::PgConnection);

/// Any error that can arise from interaction with the database.
#[non_exhaustive]
pub enum DatabaseError {
    Connection,
    Query,
}

/// Represents a new product that wants to be inserted into the database.
#[derive(Insertable)]
#[table_name = "products_schema"]
pub struct NewProduct {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) image_uri: String,
    pub(crate) price: f32,
}

/// Represents a new user that wants to be inserted into the database.
#[derive(Insertable)]
#[table_name = "users_schema"]
pub struct NewUser {
    pub(crate) first_name: String,
    pub(crate) surname: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) is_admin: bool,
}

/// Represents an update to a single product in the database.
pub struct UpdateProduct<'a> {
    pub(crate) product_id: u64,
    pub(crate) name: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
    pub(crate) price: Option<f32>,
    pub(crate) image_uri: Option<&'a str>,
}

// pub async fn update_product(
//     database: &mut WebshopDatabase,
//     update: &UpdateProduct<'_>,
// ) -> Result<Product, DatabaseError> {
//     database.transaction::<_, Error, _>(|| {
//         if let Some(name) = update.name {
//             diesel::update(products)
//                 .filter(products::product_id.eq(update.product_id))
//                 .set(schema::products::name.eq(name))
//                 .execute(&mut *database)
//                 .map_err(|_| DatabaseError::Query)?;
//
//         }
//         if let Some(description) = update.description {
//             diesel::update(products)
//                 .filter(schema::products::product_id.eq(update.product_id))
//                 .set(products::description.eq(description))
//                 .execute(&mut *database)
//                 .map_err(|_| DatabaseError::Query)?;
//         }
//         if let Some(price) = update.price {
//             diesel::update(products)
//                 .filter(products::product_id.eq(update.product_id))
//                 .set(products::description.eq(price))
//                 .execute(&mut *database)
//                 .map_err(|_| DatabaseError::Query)?;
//         }
//         if let Some(image_uri) = update.image_uri {
//             diesel::update(products)
//                 .filter(products::product_id.eq(update.product_id))
//                 .set(products::description.eq(image_uri))
//                 .execute(&mut *database)
//                 .map_err(|_| DatabaseError::Query)?;
//         }
//     });
//     products
//         .filter(products::product_id.eq(update.product_id))
//         .first::<Product>(database)
//         .map_err(|_| DatabaseError::Connection)
// }

pub async fn create_product(
    database: &mut WebshopDatabase,
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
    database: &mut WebshopDatabase,
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

pub async fn delete_user(
    database: &mut WebshopDatabase,
    user_id: i64,
) -> Result<(), DatabaseError> {
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
