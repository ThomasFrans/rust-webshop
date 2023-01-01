use crate::{database, database::WebshopDatabase, AdminGuard};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket_db_pools::Connection;

#[derive(Debug, FromForm)]
pub struct FormNewUser<'a> {
    #[field(validate = len(1..))]
    first_name: &'a str,
    surname: &'a str,
    phone: &'a str,
    email: &'a str,
    password: &'a str,
    is_admin: bool,
}

#[derive(Default, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    user_id: u64,
    first_name: String,
    surname: String,
    phone: String,
    email: String,
    password: String,
    is_active: bool,
    is_admin: bool,
}

#[post("/users/add", data = "<new_user>")]
pub async fn add(
    mut db: Connection<WebshopDatabase>,
    new_user: Form<FormNewUser<'_>>,
) -> Result<Json<u64>, Status> {
    let user_id = database::create_user(&mut db, &database::NewUser {
        first_name: new_user.first_name,
        surname: new_user.surname,
        phone: new_user.phone,
        email: new_user.email,
        password: &bcrypt::hash(new_user.password, bcrypt::DEFAULT_COST).map_err(|_| Status::BadRequest)?,
        is_admin: new_user.is_admin,
    })
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(user_id))
}

#[derive(Debug, FromForm)]
pub struct UserId(u64);

#[delete("/users/remove", data = "<id>")]
pub async fn remove(
    mut db: Connection<WebshopDatabase>,
    id: Form<UserId>,
    _admin: AdminGuard,
) -> Result<(), Status> {
    database::delete_user(&mut db, id.0)
        .await
        .map_err(|_| Status::BadRequest)
}
