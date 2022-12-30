use rocket::form::Form;
use rocket::http::{Status};
use rocket::serde::{Serialize, json::Json};
use rocket_db_pools::sqlx::{Row};
use rocket_db_pools::{sqlx, Connection};
use crate::{AdminGuard, WebshopDatabase};

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
pub async fn add(mut db: Connection<WebshopDatabase>, new_user: Form<FormNewUser<'_>>) -> Result<Json<User>, Status> {
    let row = sqlx::query!("INSERT INTO user VALUES (NULL, ?, ?, ?, ?, ?, 1, ?) RETURNING *", new_user.first_name, new_user.surname, new_user.phone, new_user.email, bcrypt::hash(new_user.password, bcrypt::DEFAULT_COST).unwrap(), new_user.is_admin)
        .fetch_one(&mut *db).await.map_err(|_| Status::from_code(400).unwrap())?;
    Ok(Json(User {
        user_id: row.get(0),
        first_name: row.get(1),
        surname: row.get(2),
        phone: row.get(3),
        email: row.get(4),
        password: row.get(5),
        is_active: row.get(6),
        is_admin: row.get(7),
    }))
}

#[derive(Debug, FromForm)]
pub struct UserId(u64);

#[delete("/users/remove", data = "<id>")]
pub async fn remove(mut db: Connection<WebshopDatabase>, id: Form<UserId>, _admin: AdminGuard) -> Result<(), Status> {
    sqlx::query!("UPDATE `user` SET `is_active` = 0 WHERE `user_id` = ?", id.0)
        .execute(&mut *db).await.map_err(|_| Status::from_code(400).unwrap())?;
    Ok(())
}