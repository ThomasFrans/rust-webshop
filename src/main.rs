#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::configuration::Configuration;
use crate::database::models::{Product, User};
use crate::database::WebshopDatabase;
use crate::schema::products as products_schema;
use crate::schema::products::dsl::products;
use crate::schema::users as users_schema;
use crate::schema::users::dsl::users;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use once_cell::sync::OnceCell;
use rocket::fs::FileServer;
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::{Redirect, Responder};
use rocket::{Build, Config, Request, Rocket};
use rocket_dyn_templates::{context, Template};
use std::process::exit;

mod api;
mod configuration;
mod database;
mod schema;

static CONFIGURATION: OnceCell<Configuration> = OnceCell::new();

#[derive(Responder)]
enum Index {
    Page(Template),
    Bootstrap(Redirect),
}

#[get("/")]
async fn index(db: WebshopDatabase, cookiejar: &CookieJar<'_>) -> Result<Index, Status> {
    if database::fetch_users(&db)
        .await
        .map_err(|_| Status::InternalServerError)?
        .is_empty()
    {
        Ok(Index::Bootstrap(Redirect::to("bootstrap")))
    } else {
        let db_products = db
            .run(|c| products.load::<Product>(c).map_err(|_| Status::BadRequest))
            .await?;
        Ok(Index::Page(Template::render(
            "index",
            context! {
                db_products,
                logged_in: cookiejar.get_private("admin").is_some(),
            },
        )))
    }
}

#[get("/bootstrap")]
async fn bootstrap() -> Template {
    Template::render("bootstrap", context! {})
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
    db: WebshopDatabase,
    cookiejar: &CookieJar<'_>,
    _userid: UserIdGuard,
    _admin: AdminGuard,
) -> Result<Template, Status> {
    let people = db
        .run(|c| {
            users
                .order(users_schema::user_id.asc())
                .load::<User>(c)
                .map_err(|_| Status::BadRequest)
        })
        .await?;

    let db_products = db
        .run(|c| {
            products
                .order(products_schema::product_id.asc())
                .load::<Product>(c)
                .map_err(|_| Status::BadRequest)
        })
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

// #[launch] creates the tokio runtime, creates the default Rocket configuration (using Figment),
// and then calls block_on on the rocket future. It wasn't magic after all :)
#[launch]
fn rocket() -> Rocket<Build> {
    CONFIGURATION
        .set(
            Configuration::new()
                .map_err(|err| {
                    println!("Error: {err}");
                    exit(1);
                })
                .unwrap(),
        )
        .unwrap();

    database::run_pending_migrations().expect("Can't run database migrations.");

    let config = Config::figment()
        .merge((
            "databases.webshop.url",
            &CONFIGURATION.get().unwrap().database_url,
        ))
        .merge((
            "address",
            &CONFIGURATION
                .get()
                .unwrap()
                .webserver_address
                .clone()
                .unwrap_or(String::from("localhost")),
        ))
        .merge((
            "port",
            &CONFIGURATION.get().unwrap().webserver_port.unwrap_or(8000),
        ))
        .merge(("secret_key", &CONFIGURATION.get().unwrap().secret_key));
    rocket::custom(config)
        .mount("/", routes![index, login, admin, bootstrap,])
        .mount(
            "/api/",
            routes![
                api::users::add,
                api::users::remove,
                api::login::login,
                api::login::logout,
                api::products::add,
                api::products::edit
            ],
        )
        .mount("/static/", FileServer::from("static/"))
        .register("/", catchers![unauthorized])
        .attach(Template::fairing())
        .attach(WebshopDatabase::fairing())
}
