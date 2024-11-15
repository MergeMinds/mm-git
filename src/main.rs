mod routes;
mod repo_manager;
mod context;
mod models;

use simplelog::*;
use sqlx::postgres::PgPoolOptions;
use actix_web::{middleware::Logger, web::Data, App, HttpServer};


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    ).unwrap();


    let db_url = std::env::var("DATABASE_URL")?;
    let db = PgPoolOptions::new().connect(&db_url).await?;
    let ctx = context::Context { db };

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(ctx.clone()))
            .configure(routes::router)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 1337))?
    .run()
    .await?;

    Ok(())
}
