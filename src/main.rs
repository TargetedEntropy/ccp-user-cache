mod config;
mod db;
mod dtos;
mod error;
mod extractors;
mod models;
mod scopes;
mod utils;

use actix_cors::Cors;
use actix_web::{
    get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder,
};
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
    
    // Migrations
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations executed successfully."),
        Err(e) => eprintln!("Error executing migrations: {}", e),
    };

    // Create the client
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

    // HTTP Server with ALLOWED ORIGINS, #fancy
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(scopes::auth::auth_scope())
            .service(scopes::users::users_scope())
            .service(health_checker_handler)
    })
    .bind(("0.0.0.0", config.port))? // why are we binding to * when we only allow localhost, cause yee haw thats why.
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
