extern crate actix;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

mod actors;
mod models;
mod schema;

use actix::SyncArbiter;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actors::db::{CreateWord, DbActor, DeleteWord, GetWord, UpdateWord};
use diesel::{
  connection::Connection,
  r2d2::{ConnectionManager, Pool},
  PgConnection,
};
use dotenv::dotenv;
use models::AppState;
use std::env;

fn run_migrations(db_url: &str) {
  embed_migrations!();
  let connection = PgConnection::establish(db_url).expect("Error connecting to database");
  embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
    .expect("Error running migrations");
}

fn get_pool(db_url: &str) -> Pool<ConnectionManager<PgConnection>> {
  let manager = ConnectionManager::<PgConnection>::new(db_url);
  Pool::builder()
    .build(manager)
    .expect("Error building a connection pool")
}

#[get("/api/word/{id}")]
async fn get_word(id: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {
  let db = &state.as_ref().db.clone();
  let id = id.into_inner();
  match db.send(GetWord { id: id }).await {
    Ok(Ok(word)) => HttpResponse::Ok().json(word),
    Ok(Err(_)) => HttpResponse::NotFound().json("Word not found"),
    _ => HttpResponse::InternalServerError().json("An error occurred"),
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let db_url = env::var("DATABASE_URL").expect("Error retrieving database url");
  run_migrations(&db_url);
  let pool = get_pool(&db_url);
  let db_addr = SyncArbiter::start(5, move || DbActor(pool.clone()));

  HttpServer::new(move || {
    App::new().service(get_word).app_data(AppState {
      db: db_addr.clone(),
    })
  })
  .bind(("0.0.0.0", 5000))?
  .run()
  .await
}
