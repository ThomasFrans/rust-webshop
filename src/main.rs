#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::database::models::{Product, User};
use crate::database::WebshopDatabase;
use crate::schema::products::dsl::products;
use crate::schema::users::dsl::users;
use diesel::RunQueryDsl;
use rocket::fs::FileServer;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{Build, Request, Rocket};
use rocket_dyn_templates::{context, Template};

mod api;
mod database;
mod ext_traits;
mod schema;

#[get("/")]
async fn index(mut db: WebshopDatabase, cookiejar: &CookieJar<'_>) -> Result<Template, Status> {
    let db_products = db
        .run(|c| products.load::<Product>(c).map_err(|_| Status::BadRequest))
        .await?;
    Ok(Template::render(
        "index",
        context! {
            db_products,
            logged_in: cookiejar.get_private("admin").is_some(),
        },
    ))
}

#[get("/login")]
async fn login(cookiejar: &CookieJar<'_>) -> Template {
    Template::render(
        "login",
        context! {
            logged_in: cookiejar.get_private("userid").is_some()
        },
    )
}

pub struct AdminGuard;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = AdminKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(_cookie) = cookies.get_private("admin") {
            Outcome::Success(AdminGuard)
        } else {
            Outcome::Failure((Status::Unauthorized, AdminKeyError::Invalid))
        }
    }
}

pub struct UserIdGuard(String);

#[derive(Debug)]
pub enum AdminKeyError {
    Invalid,
    Missing,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserIdGuard {
    type Error = AdminKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(cookie) = cookies.get_private("userid") {
            Outcome::Success(UserIdGuard(cookie.value().to_owned()))
        } else {
            Outcome::Failure((Status::Unauthorized, AdminKeyError::Missing))
        }
    }
}

#[get("/admin")]
async fn admin(
    mut db: WebshopDatabase,
    cookiejar: &CookieJar<'_>,
    _userid: UserIdGuard,
    _admin: AdminGuard,
) -> Result<Template, Status> {
    let people = db
        .run(|c| users.load::<User>(c).map_err(|_| Status::BadRequest))
        .await?;

    let db_products = db
        .run(|c| products.load::<Product>(c).map_err(|_| Status::BadRequest))
        .await?;
    Ok(Template::render(
        "admin",
        context! {
            people,
            db_products,
            logged_in: cookiejar.get_private("admin").is_some()
        },
    ))
}

#[catch(401)]
async fn unauthorized() -> Template {
    Template::render("error/401", context! {})
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, login, admin])
        .mount(
            "/api/",
            routes![
                api::users::add,
                api::users::remove,
                api::login::login,
                api::login::logout,
                // api::products::add,
                // api::products::edit
            ],
        )
        .mount("/static/", FileServer::from("static/"))
        .register("/", catchers![unauthorized])
        .attach(Template::fairing())
        .attach(WebshopDatabase::fairing())
}
