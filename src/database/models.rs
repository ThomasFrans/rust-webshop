//! The Rust representation of the database schemas.
use rocket::serde::Serialize;

// Queryable: Allows the value to be loaded with a `load<T>()` method.
// Insertable: Allows the value to be passed to a `values()` method on an insert_into statement.
#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: i64,
    pub first_name: String,
    pub surname: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub is_admin: bool,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub product_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub image_uri: String,
    pub is_active: bool,
    pub price: f32,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    pub item_id: i64,
    pub product_id: i64,
}
