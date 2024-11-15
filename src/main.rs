mod context;
mod git;
mod models;
mod repo_manager;
mod routes;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use simplelog::*;
use sqlx::postgres::PgPoolOptions;
use std::{fs::File, path::PathBuf};

const SHELL_NAME: &str = "mmshell";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    _ = dotenvy::dotenv();

    // TODO(guschin): use more consisten approach to choose between shell or service mode
    // NOTE(guschin): this doesn't work with symlinks on linux
    let exe_name = std::env::current_exe()
        .unwrap()
        .file_name()
        .unwrap()
        .to_owned();

    CombinedLogger::init(vec![
        TermLogger::new(
            if exe_name == SHELL_NAME {
                LevelFilter::Off
            } else {
                LevelFilter::Info
            },
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("mm-git.log").unwrap(),
        ),
    ])
    .unwrap();

    let home = PathBuf::from(
        std::env::var("HOME").expect("Couldn't get $HOME variable"),
    );
    let repos_path = home.join("repos");
    let repo_manager = git::RepoManager::new(repos_path);

    let db_url = std::env::var("DATABASE_URL")?;
    let db = PgPoolOptions::new().connect(&db_url).await?;
    let ctx = context::Context { db, repo_manager };

    if exe_name == SHELL_NAME {
        git::shell(ctx.clone()).await;
    } else {
        HttpServer::new(move || {
            App::new()
                .app_data(Data::new(ctx.clone()))
                .configure(routes::router)
                .wrap(Logger::default())
        })
        .bind(("0.0.0.0", 1337))?
        .run()
        .await?;
    }

    Ok(())
}
