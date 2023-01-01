use rocket::serde::Serialize;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::{Acquire, Row};

#[derive(Database)]
#[database("mysql_webshop")]
pub struct WebshopDatabase(sqlx::MySqlPool);

#[non_exhaustive]
pub enum DatabaseError {
    Connection,
    Query,
}

/// Represents a single product.
#[derive(Default, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub(crate) product_id: u64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_uri: String,
    pub(crate) is_active: bool,
    pub(crate) price: f32,
}

/// Represents an update to a single product.
#[derive(FromForm)]
pub struct UpdateProduct<'a> {
    pub(crate) product_id: u64,
    pub(crate) name: Option<&'a str>,
    pub(crate) description: Option<&'a str>,
    pub(crate) price: Option<f32>,
    pub(crate) image_uri: Option<&'a str>,
}

pub async fn update_product(
    database: &mut Connection<WebshopDatabase>,
    update: &UpdateProduct<'_>,
) -> Result<Product, DatabaseError> {
    let mut transaction = (**database)
        .begin()
        .await
        .map_err(|_| DatabaseError::Connection)?;
    if let Some(name) = update.name {
        sqlx::query!(
            "UPDATE `product` SET `name` = ? WHERE `product_id` = ?",
            name,
            update.product_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| DatabaseError::Query)?;
    }
    if let Some(description) = update.description {
        sqlx::query!(
            "UPDATE `product` SET `description` = ? WHERE `product_id` = ?",
            description,
            update.product_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| DatabaseError::Query)?;
    }
    if let Some(price) = update.price {
        sqlx::query!(
            "UPDATE `product` SET `price` = ? WHERE `product_id` = ?",
            price,
            update.product_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| DatabaseError::Query)?;
    }
    if let Some(image_uri) = update.image_uri {
        sqlx::query!(
            "UPDATE `product` SET `image_uri` = ? WHERE `product_id` = ?",
            image_uri,
            update.product_id
        )
        .execute(&mut *transaction)
        .await
        .map_err(|_| DatabaseError::Query)?;
    }
    transaction
        .commit()
        .await
        .map_err(|_| DatabaseError::Connection)?;
    sqlx::query!(
        "SELECT * FROM `product` WHERE `product_id` = ?",
        update.product_id
    )
    .fetch_one(&mut (**database))
    .await
    .map_err(|_| DatabaseError::Query)
    .map(|row| Product {
        product_id: row.product_id,
        name: row.name,
        description: row.description,
        image_uri: row.image_uri,
        is_active: row.is_active != 0,
        price: row.price,
    })
}

pub struct NewProduct<'a> {
    pub(crate) name: &'a str,
    pub(crate) description: &'a str,
    pub(crate) image_uri: &'a str,
    pub(crate) price: f32,
}

pub async fn create_product(database: &mut Connection<WebshopDatabase>, product: NewProduct<'_>) -> Result<u64, DatabaseError> {
    sqlx::query!(
        "INSERT INTO `product` VALUES (NULL, ?, ?, ?, 1, ?) RETURNING product_id",
        product.name,
        product.description,
        product.image_uri,
        product.price
    )
        .fetch_one(&mut **database)
        .await
        .map_err(|_| DatabaseError::Query)
        .map(|row| row.get(0))
}

#[derive(Serialize, Default)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub(crate) user_id: u64,
    pub(crate) first_name: String,
    pub(crate) surname: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) is_active: bool,
    pub(crate) is_admin: bool,
}

pub async fn user_with_email(
    database: &mut Connection<WebshopDatabase>,
    email: &str,
) -> Result<User, DatabaseError> {
    sqlx::query!("SELECT * FROM `user` WHERE `email` = ?", email)
        .fetch_one(&mut **database)
        .await
        .map_err(|_| DatabaseError::Query)
        .map(|row| User {
            user_id: row.user_id,
            first_name: row.first_name,
            surname: row.surname,
            phone: row.phone,
            email: row.email,
            password: row.password,
            is_active: row.is_active != 0,
            is_admin: row.is_admin != 0,
        })
}

pub async fn delete_user(
    database: &mut Connection<WebshopDatabase>,
    user_id: u64,
) -> Result<(), DatabaseError> {
    sqlx::query!(
        "UPDATE `user` SET `is_active` = 0 WHERE `user_id` = ?",
        user_id)
        .execute(&mut **database)
        .await
        .map_err(|_| DatabaseError::Query)
        .map(|_| ())
}

pub struct NewUser<'a> {
    pub(crate) first_name: &'a str,
    pub(crate) surname: &'a str,
    pub(crate) phone: &'a str,
    pub(crate) email: &'a str,
    pub(crate) password: &'a str,
    pub(crate) is_admin: bool,
}

pub async fn create_user(
    database: &mut Connection<WebshopDatabase>,
    user: &NewUser<'_>,
) -> Result<u64, DatabaseError> {
    let row = sqlx::query!(
            "INSERT INTO user VALUES (NULL, ?, ?, ?, ?, ?, 1, ?) RETURNING `user_id`",
            user.first_name,
            user.surname,
            user.phone,
            user.email,
            user.password,
            user.is_admin
        )
        .fetch_one(&mut **database)
        .await
        .map_err(|_| DatabaseError::Query)?;
    Ok(row.get(0))
}