//! The Rust representation of the database schemas.
use diesel::data_types::PgNumeric;
use rocket::serde::Serialize;

// Queryable: Allows the value to be loaded with a `load<T>()` method.
// Insertable: Allows the value to be passed to a `values()` method on an insert_into statement.
#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub(crate) user_id: i64,
    pub(crate) first_name: String,
    pub(crate) surname: String,
    pub(crate) phone: String,
    pub(crate) email: String,
    pub(crate) password: String,
    pub(crate) is_active: bool,
    pub(crate) is_admin: bool,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Product {
    pub(crate) product_id: i64,
    pub(crate) name: String,
    pub(crate) description: Option<String>,
    pub(crate) image_uri: String,
    pub(crate) is_active: bool,
    pub(crate) price: f32,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Item {
    pub(crate) item_id: i64,
    pub(crate) product_id: i64,
}
