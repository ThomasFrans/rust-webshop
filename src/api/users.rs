use crate::database::models::User;
use crate::database::NewUser;
use crate::{database, database::WebshopDatabase, AdminGuard};
use rocket::form::Form;
use rocket::http::Status;
use rocket::serde::json::Json;

#[derive(FromForm)]
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

#[derive(FromForm)]
pub struct UserId(i64);

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

#[post("/users/add", data = "<new_user>")]
pub async fn add(
    database: WebshopDatabase,
    new_user: Form<FormNewUser<'_>>,
) -> Result<Json<User>, Status> {
    let user = database::create_user(&database, new_user.into_inner().into())
        .await
        .map_err(|_| Status::BadRequest)?;
    Ok(Json(user))
}

#[delete("/users/remove", data = "<id>")]
pub async fn remove(
    database: WebshopDatabase,
    id: Form<UserId>,
    _admin: AdminGuard,
) -> Result<(), Status> {
    database::delete_user(&database, id.0)
        .await
        .map_err(|_| Status::BadRequest)
}
