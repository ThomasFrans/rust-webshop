use crate::database::{user_with_email, WebshopDatabase};
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket_db_pools::Connection;

#[derive(Debug, FromForm)]
pub struct LoginData<'a> {
    email: &'a str,
    password: &'a str,
}

#[post("/login", data = "<login>")]
pub async fn login(
    mut db: Connection<WebshopDatabase>,
    cookiejar: &CookieJar<'_>,
    login: Form<LoginData<'_>>,
) -> Result<(), Status> {
    let user_row = user_with_email(&mut db, login.email)
        .await
        .map_err(|_| Status::InternalServerError)?;
    if bcrypt::verify(login.password, &user_row.password)
        .map_err(|_| Status::InternalServerError)?
    {
        cookiejar.add_private(Cookie::new("userid", user_row.user_id.to_string()));
        if user_row.is_admin {
            cookiejar.add_private(Cookie::new("admin", "true"));
        }
        Ok(())
    } else {
        Err(Status::BadRequest)
    }
}

#[post("/logout")]
pub async fn logout(cookiejar: &CookieJar<'_>) {
    if let Some(cookie) = cookiejar.get_private("userid") {
        cookiejar.remove_private(cookie);
    }
    if let Some(cookie) = cookiejar.get_private("admin") {
        cookiejar.remove_private(cookie);
    }
}
