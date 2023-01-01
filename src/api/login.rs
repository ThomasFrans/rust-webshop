use crate::database::WebshopDatabase;
use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket_db_pools::{sqlx, Connection};

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
    let user_row = sqlx::query!(
        "SELECT `user_id`, `email`, `password`, `is_admin` FROM `user` WHERE `email` = ?",
        login.email
    )
    .fetch_one(&mut *db)
    .await
    .map_err(|_| Status::from_code(400).unwrap())?;
    if bcrypt::verify(login.password, &user_row.password)
        .map_err(|_| Status::from_code(500).unwrap())?
    {
        cookiejar.add_private(Cookie::new("userid", user_row.user_id.to_string()));
        if user_row.is_admin != 0 {
            cookiejar.add_private(Cookie::new("admin", "true"));
        }
        Ok(())
    } else {
        Err(Status::from_code(400).unwrap())
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
