use crate::database::models::Product;
use crate::database::{
    create_product, update_product, DatabaseError, NewProduct, UpdateProduct, WebshopDatabase,
};
use crate::AdminGuard;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;

#[derive(FromForm)]
pub struct FormNewProduct<'a> {
    name: &'a str,
    description: &'a str,
    price: f32,
    image_uri: &'a str,
}

#[derive(FromForm)]
pub struct FormUpdateProduct<'a> {
    product_id: i64,
    name: Option<&'a str>,
    description: Option<&'a str>,
    image_uri: Option<&'a str>,
    price: Option<f32>,
}

impl From<FormNewProduct<'_>> for NewProduct {
    fn from(value: FormNewProduct) -> Self {
        Self {
            name: value.name.to_owned(),
            description: value.description.to_owned(),
            image_uri: value.image_uri.to_owned(),
            price: value.price,
        }
    }
}

impl From<FormUpdateProduct<'_>> for UpdateProduct {
    fn from(value: FormUpdateProduct<'_>) -> Self {
        Self {
            product_id: value.product_id,
            name: value.name.map(|i| i.to_owned()),
            description: value.description.map(|i| i.to_owned()),
            price: value.price,
            image_uri: value.image_uri.map(|i| i.to_owned()),
        }
    }
}

#[post("/products/add", data = "<product>")]
pub async fn add(
    product: Form<FormNewProduct<'_>>,
    database: WebshopDatabase,
    _admin: AdminGuard,
) -> Result<Json<Product>, Status> {
    let new_product = create_product(&database, product.into_inner().into())
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(new_product))
}

#[post("/products/edit", data = "<update>")]
pub async fn edit(
    update: Form<FormUpdateProduct<'_>>,
    database: WebshopDatabase,
    _admin: AdminGuard,
) -> Result<Json<Product>, Status> {
    update_product(&database, update.into_inner().into())
        .await
        .map_err(|error| match error {
            DatabaseError::Query => Status::BadRequest,
        })
        .map(Json)
}
