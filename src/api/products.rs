use crate::database::{create_product, DatabaseError, Product, UpdateProduct, WebshopDatabase};
use crate::{database, AdminGuard};
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

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
) -> Result<Json<u64>, Status> {
    let product_id = create_product(&mut database, database::NewProduct {
        name: product.name,
        description: product.description,
        price: product.price,
        image_uri: product.image_uri,
    })
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(product_id))
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
