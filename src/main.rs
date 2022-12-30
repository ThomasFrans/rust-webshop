#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket::{Build, Request, Rocket};
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::sqlx::{Row};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_dyn_templates::{context, Template};

mod api;

#[derive(Database)]
#[database("mysql_webshop")]
pub struct WebshopDatabase(sqlx::MySqlPool);

#[get("/")]
async fn index(mut db: Connection<WebshopDatabase>, cookiejar: &CookieJar<'_>) -> Template {
    let products = sqlx::query("SELECT * FROM `product`")
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            context! {
                id: row.get::<u64, _>("product_id"),
                name: row.get::<String, _>("name"),
                description: row.get::<String, _>("description"),
                price: row.get::<f32, _>("price"),
                image_uri: row.get::<String, _>("image_uri"),
                is_active: row.get::<bool, _>("is_active"),
            }
        })
        .collect::<Vec<_>>();
    Template::render(
        "index",
        context! {
            products,
            logged_in: cookiejar.get_private("admin").is_some(),
        },
    )
}

#[get("/login")]
async fn login(cookiejar: &CookieJar<'_>) -> Template {
    Template::render(
        "login",
        context! {
            logged_in: cookiejar.get_private("admin").is_some()
        },
    )
}

pub struct AdminGuard();

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = AdminKeyError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let cookies = request.cookies();
        if let Some(_cookie) = cookies.get_private("admin") {
            Outcome::Success(AdminGuard())
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
async fn admin(mut db: Connection<WebshopDatabase>, cookiejar: &CookieJar<'_>, userid: UserIdGuard, _admin: AdminGuard) -> Result<Template, Status> {
    println!("{}", userid.0);
    let people = sqlx::query("SELECT * FROM user")
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            context!(
                id: row.get::<u64, _>("user_id"),
                first_name: row.get::<String, _>("first_name"),
                surname: row.get::<String, _>("surname"),
                phone: row.get::<String, _>("phone"),
                email: row.get::<String, _>("email"),
                password: row.get::<String, _>("password"),
                is_active: row.get::<bool, _>("is_active"),
                is_admin: row.get::<bool, _>("is_admin")
            )
        })
        .collect::<Vec<_>>();
    let products = sqlx::query("SELECT * FROM `product`")
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .into_iter()
        .map(|row| {
            context! {
                id: row.get::<u64, _>("product_id"),
                name: row.get::<String, _>("name"),
                description: row.get::<String, _>("description"),
                price: row.get::<f32, _>("price"),
                image_uri: row.get::<String, _>("image_uri"),
                is_active: row.get::<bool, _>("is_active"),
            }
        })
        .collect::<Vec<_>>();
    Ok(Template::render(
        "admin",
        context! {
            people,
            products,
            logged_in: cookiejar.get_private("admin").is_some()
        }
    ))
}

#[catch(401)]
async fn unauthorized() -> Template {
    Template::render(
        "error/401",
        context! {}
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, login, admin])
        .mount("/api/", routes![api::users::add, api::users::remove, api::login::login, api::login::logout, api::products::add])
        .mount("/static/", FileServer::from("static/"))
        .register("/", catchers![unauthorized])
        .attach(Template::fairing())
        .attach(WebshopDatabase::init())
}
