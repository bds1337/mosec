use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error, error,
    middleware::{self, ErrorHandlerResponse, ErrorHandlers},
};
use actix_files::{Files};
use dotenvy::dotenv;
use system::get_host_info;
use std::env;

use tera::{Context, Tera};

mod database;
mod model;
mod system;

use sqlx::SqlitePool;

// use crate::database;

async fn network(tmpl: web::Data<tera::Tera>, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let sys = system::init_info().unwrap();
    let nw = system::get_network_info(&sys).unwrap();

    let mut ctx = Context::new();
    ctx.insert("networks", &nw);

    let rendered = tmpl.render("network.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn devices(tmpl: web::Data<tera::Tera>, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let rendered = tmpl.render("devices.html", &Context::new())
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn audit(tmpl: web::Data<tera::Tera>, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let audits = database::get_all_audits(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;

    let mut ctx = Context::new();
    ctx.insert("audits", &audits);

    let rendered = tmpl.render("audit.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn index(tmpl: web::Data<tera::Tera>, pool: web::Data<SqlitePool>) -> Result<HttpResponse, Error> {
    let audits = database::get_all_audits(&pool)
        .await
        .map_err(error::ErrorInternalServerError)?;
    
    let sys = system::init_info().unwrap();

    let hst = system::get_host_info(&sys).unwrap();
    //print!("nw: {:?}\n", nw);
    //print!("au: {:?}\n", audits);

    let mut ctx = Context::new();
    ctx.insert("audits", &audits);
    ctx.insert("host", &hst);

    // let rendered = tmpl.render("index.html", &tera::Context::new())
    let rendered = tmpl.render("index.html", &ctx)
        .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // std::env::set_var("RUST_LOG", "actix_web=info");
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    for (key, value) in env::vars() {
        println!("- {key}: {value}");
    }


    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = database::init_pool(&database_url)
        .await
        .expect("Failed to create a database pool");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default()) // enable logger
            // FIXME!: Stupid shit css include (use env or smth)
            .service(Files::new("/static", "static").show_files_listing())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/audit").route(web::get().to(audit)))
            .service(web::resource("/devices").route(web::get().to(devices)))
            .service(web::resource("/network").route(web::get().to(network)))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
