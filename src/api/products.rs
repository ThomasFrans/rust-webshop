use rocket::form::{Form, FromForm};
use rocket::http::{CookieJar, Status};
use rocket::serde::{Serialize, json::Json};
use rocket_db_pools::sqlx::{Row};
use rocket_db_pools::{sqlx, Connection};
use crate::{AdminGuard, WebshopDatabase};

#[derive(FromForm, Default)]
pub struct NewProduct<'a> {
    name: &'a str,
    description: &'a str,
    price: f32,
    image_uri: &'a str,
}

#[derive(Default, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    product_id: u64,
    name: String,
    description: String,
    image_uri: String,
    is_active: bool,
    price: f32,
}

#[post("/products/add", data = "<product>")]
pub async fn add(product: Form<NewProduct<'_>>, mut database: Connection<WebshopDatabase>, cookiejar: &CookieJar<'_>, _admin: AdminGuard) -> Result<Json<Product>, Status> {
    if cookiejar.get_private("userid").is_none() {
        return Err(Status::from_code(401).unwrap());
    }
    let row = sqlx::query!("INSERT INTO `product` VALUES (NULL, ?, ?, ?, 1, ?) RETURNING *", product.name, product.description, product.image_uri, product.price)
        .fetch_one(&mut *database).await.map_err(|_| Status::from_code(400).unwrap())?;
    Ok(Json(Product {
        product_id: row.get(0),
        name: row.get(1),
        description: row.get(2),
        image_uri: row.get(3),
        is_active: row.get(4),
        price: row.get(5),
    }))
}

pub async fn fetch() {

}