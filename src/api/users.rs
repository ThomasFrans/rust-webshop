use crate::database::models::User;
use crate::database::NewUser;
use crate::schema::users as users_schema;
use crate::{database, database::WebshopDatabase, AdminGuard};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};

#[derive(Debug, FromForm, Insertable)]
#[table_name = "users_schema"]
pub struct FormNewUser<'a> {
    #[field(validate = len(1..))]
    first_name: &'a str,
    #[field(validate = len(1..))]
    surname: &'a str,
    phone: &'a str,
    #[field(validate = len(3..255))]
    email: &'a str,
    #[field(validate = len(8..))]
    password: &'a str,
    is_admin: bool,
}

impl From<FormNewUser<'_>> for NewUser {
    fn from(value: FormNewUser) -> Self {
        Self {
            first_name: value.first_name.to_owned(),
            surname: value.surname.to_owned(),
            phone: value.phone.to_owned(),
            email: value.email.to_owned(),
            password: bcrypt::hash(value.password, bcrypt::DEFAULT_COST).unwrap(),
            is_admin: value.is_admin,
        }
    }
}

// impl From<FormNewUser<'_>> for database::NewUser {
//     fn from(value: FormNewUser) -> Self {
//         Self {
//             first_name: value.first_name,
//             surname: value.surname,
//             phone: value.phone,
//             email: value.email,
//             password: &bcrypt::hash(value.password, bcrypt::DEFAULT_COST).unwrap(),
//             is_admin: value.is_admin,
//         }
//     }
// }
//
// #[derive(Default, Serialize)]
// #[serde(crate = "rocket::serde")]
// pub struct User {
//     user_id: u64,
//     first_name: String,
//     surname: String,
//     phone: String,
//     email: String,
//     password: String,
//     is_active: bool,
//     is_admin: bool,
// }

#[post("/users/add", data = "<new_user>")]
pub async fn add(
    mut db: WebshopDatabase,
    new_user: Form<FormNewUser<'_>>,
) -> Result<Json<User>, Status> {
    let user = database::create_user(&mut db, new_user.into_inner().into())
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(user))
}

#[derive(Debug, FromForm)]
pub struct UserId(i64);

#[delete("/users/remove", data = "<id>")]
pub async fn remove(
    mut db: WebshopDatabase,
    id: Form<UserId>,
    _admin: AdminGuard,
) -> Result<(), Status> {
    database::delete_user(&mut db, id.0)
        .await
        .map_err(|_| Status::BadRequest)
}
