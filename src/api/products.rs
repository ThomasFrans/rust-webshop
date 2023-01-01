use crate::database::{DatabaseError, Product, UpdateProduct, WebshopDatabase};
use crate::{database, AdminGuard};
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::sqlx::Row;
use rocket_db_pools::{sqlx, Connection};

#[derive(FromForm, Default)]
pub struct NewProduct<'a> {
    name: &'a str,
    description: &'a str,
    price: f32,
    image_uri: &'a str,
}

#[post("/products/add", data = "<product>")]
pub async fn add(
    product: Form<NewProduct<'_>>,
    mut database: Connection<WebshopDatabase>,
    _admin: AdminGuard,
) -> Result<Json<Product>, Status> {
    let row = sqlx::query!(
        "INSERT INTO `product` VALUES (NULL, ?, ?, ?, 1, ?) RETURNING *",
        product.name,
        product.description,
        product.image_uri,
        product.price
    )
    .fetch_one(&mut *database)
    .await
    .map_err(|_| Status::from_code(400).unwrap())?;
    Ok(Json(Product {
        product_id: row.get(0),
        name: row.get(1),
        description: row.get(2),
        image_uri: row.get(3),
        is_active: row.get(4),
        price: row.get(5),
    }))
}

#[post("/products/edit", data = "<update>")]
pub async fn edit(
    update: Form<UpdateProduct<'_>>,
    mut database: Connection<WebshopDatabase>,
    _admin: AdminGuard,
) -> Result<Json<Product>, Status> {
    database::update_product(&mut database, &update)
        .await
        .map_err(|error| match error {
            DatabaseError::Query => todo!(),
            DatabaseError::Connection => todo!(),
        })
        .map(Json)
}
