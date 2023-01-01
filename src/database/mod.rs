use rocket::serde::Serialize;
use rocket_db_pools::{sqlx, Connection, Database};
use sqlx::Acquire;

#[derive(Database)]
#[database("mysql_webshop")]
pub struct WebshopDatabase(sqlx::MySqlPool);

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

#[non_exhaustive]
pub enum DatabaseError {
    Connection,
    Query,
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
