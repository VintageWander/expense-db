#![allow(dead_code, unused_variables)]

use dotenv::dotenv;
use error::Error;
use handler::{create_trips_handler, hello};
use repo::ExpenseRepo;
use salvo::{affix, prelude::TcpListener, Router, Server};
use web::Web;

use crate::mongo::DB;

mod check;
mod error;
mod handler;
mod helper;
mod model;
mod mongo;
mod repo;
mod validation;
mod web;

type Result<T> = std::result::Result<T, Error>;
type WebResult = Result<Web>;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;
    let db = DB::init().await?;
    let expenses_repo = ExpenseRepo::init(&db);

    let router = Router::with_hoop(affix::insert("expenses_repo", expenses_repo))
        .get(hello)
        .post(create_trips_handler);

    let port = std::env::var("PORT")?;
    let listener = TcpListener::bind(&format!("127.0.0.1:{port}"));

    Server::new(listener).serve(router).await;
    Ok(())
}
