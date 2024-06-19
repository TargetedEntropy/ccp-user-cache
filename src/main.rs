mod config;
mod db;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use config::Config;
use db::DBClient;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[derive(Debug, Clone)]
pub struct AppState {
    pub env: Config,
    pub db_client: DBClient,
}

// OMG WE HAVE A MAIN
#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for ENV Logging
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // Lets get that .env file
    dotenv().ok();
    env_logger::init();

    // Config settings, we need these now that we have read .env
    let config = Config::init();

    // DABATABSE STUFF, should be oracle, smh
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let db_client = DBClient::new(pool);

    // Start the AppState
    let app_state: AppState = AppState {
        env: config.clone(),
        db_client,
    };

    // YO its live
    println!(
        "{}",
        format!("Server is running on http://localhost:{}", config.port)
    );

    // START THE SERVER GROG
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(Logger::default())
            .service(health_checker_handler)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await?;

    Ok(())
}


// AM I A REAL API
#[get("/api/healthchecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Complete Restful API in Rust";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}
