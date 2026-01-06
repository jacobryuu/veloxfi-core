use crate::core::engine::Engine;
use actix_web::{web, App, HttpServer};
use log::info;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub mod handlers;

pub struct AppState {
    pub engine: Mutex<Engine>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct PlaceOrderRequest {
    pub order_id: u64,
    pub account: String,
    pub market: String,
    pub side: String,
    pub price: u64,
    pub quantity: u64,
    pub timestamp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct PlaceOrderResponse {
    pub success: bool,
    pub message: String,
    pub trades: Option<Vec<TradeResponse>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct TradeResponse {
    pub trade_id: u64,
    pub buy_order_id: u64,
    pub sell_order_id: u64,
    pub price: u64,
    pub qty: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CreateAccountRequest {
    pub account_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct CreateAccountResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct DepositRequest {
    pub account_id: String,
    pub asset: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct DepositResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct BalanceResponse {
    pub account_id: String,
    pub asset: String,
    pub balance: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[cfg_attr(feature = "swagger", derive(utoipa::ToSchema))]
pub struct PositionResponse {
    pub account: String,
    pub market: String,
    pub quantity: i128,
    pub avg_entry_price: Option<u64>,
    pub realized_pnl: i128,
}

pub async fn start_server(engine: Engine, host: &str, port: u16) -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        engine: Mutex::new(engine),
    });

    info!("Starting HTTP server on {}:{}", host, port);

    #[cfg(feature = "swagger")]
    info!(
        "Swagger UI available at http://{}:{}/swagger-ui/",
        host, port
    );

    let server = HttpServer::new(move || {
        let mut app = App::new().app_data(app_state.clone()).service(
            web::scope("/api")
                .route("/health", web::get().to(handlers::health_check))
                .route("/markets", web::post().to(handlers::create_market))
                .route("/accounts", web::post().to(handlers::create_account))
                .route(
                    "/accounts/{account_id}/deposit",
                    web::post().to(handlers::deposit),
                )
                .route(
                    "/accounts/{account_id}/balance/{asset}",
                    web::get().to(handlers::get_balance),
                )
                .route("/orders/place", web::post().to(handlers::place_order))
                .route(
                    "/positions/{account_id}/{market}",
                    web::get().to(handlers::get_position),
                ),
        );

        #[cfg(feature = "swagger")]
        {
            use utoipa::OpenApi;
            use utoipa_swagger_ui::SwaggerUi;

            // OpenAPI specification
            #[derive(utoipa::OpenApi)]
            #[openapi(
                info(title = "veloxfi-core", version = "0.1.0"),
                paths(
                    handlers::health_check,
                    handlers::create_market,
                    handlers::create_account,
                    handlers::deposit,
                    handlers::get_balance,
                    handlers::place_order,
                    handlers::get_position,
                ),
                components(
                    schemas(
                        PlaceOrderRequest,
                        PlaceOrderResponse,
                        TradeResponse,
                        CreateAccountRequest,
                        CreateAccountResponse,
                        DepositRequest,
                        DepositResponse,
                        BalanceResponse,
                        PositionResponse,
                    )
                ),
                tags(
                    (name = "orders", description = "Order management endpoints"),
                    (name = "accounts", description = "Account management endpoints"),
                )
            )]
            struct ApiDoc;

            app = app.service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            );
        }

        app
    })
    .bind((host, port))?
    .run()
    .await;

    server
}
