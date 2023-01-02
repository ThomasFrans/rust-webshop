use crate::database::models::Product;
use crate::database::{create_product, DatabaseError, UpdateProduct, WebshopDatabase};
use crate::schema::products as products_schema;
use crate::{database, AdminGuard};
use diesel::data_types::PgNumeric;
use rocket::form::{Form, FromForm};
use rocket::http::Status;
use rocket::serde::json::Json;

// #[derive(Insertable, FromForm, Default)]
// #[table_name = "products_schema"]
// pub struct NewProduct<'a> {
//     name: &'a str,
//     description: &'a str,
//     price: f64,
//     image_uri: &'a str,
// }
//
// #[post("/products/add", data = "<product>")]
// pub fn add(
//     product: Form<NewProduct<'_>>,
//     mut database: WebshopDatabase,
//     _admin: AdminGuard,
// ) -> Result<Json<u64>, Status> {
//     let product_id = create_product(&mut database, &  *product)
//         .map_err(|_| Status::BadRequest)?;
//     Ok(Json(product_id))
// }
//
// #[post("/products/edit", data = "<update>")]
// pub fn edit(
//     update: Form<UpdateProduct<'_>>,
//     mut database: WebshopDatabase,
//     _admin: AdminGuard,
// ) -> Result<Json<Product>, Status> {
//     database::update_product(&mut database, &update)
//         .map_err(|error| match error {
//             DatabaseError::Query => todo!(),
//             DatabaseError::Connection => todo!(),
//         })
//         .map(Json)
// }
