use crate::api::{
    AppState, BalanceResponse, CreateAccountRequest, CreateAccountResponse, DepositRequest,
    DepositResponse, PlaceOrderRequest, PlaceOrderResponse, PositionResponse, TradeResponse,
};
use crate::core::market::Market;
use crate::core::order::{Order, Side};
use actix_web::{web, HttpResponse, Result as ActixResult};
use log::{error, info};

#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "Server is healthy", body = serde_json::Value)
    )
))]
pub async fn health_check() -> ActixResult<HttpResponse> {
    info!("Health check requested");
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "veloxfi-core is running"
    })))
}

#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/api/markets",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Market created successfully", body = serde_json::Value)
    )
))]
pub async fn create_market(
    state: web::Data<AppState>,
    req: web::Json<serde_json::Value>,
) -> ActixResult<HttpResponse> {
    let symbol = req
        .get("symbol")
        .and_then(|v| v.as_str())
        .unwrap_or("BTC-USD");
    let base = req.get("base").and_then(|v| v.as_str()).unwrap_or("BTC");
    let quote = req.get("quote").and_then(|v| v.as_str()).unwrap_or("USD");
    let precision = req.get("precision").and_then(|v| v.as_u64()).unwrap_or(1);

    let market = Market::new(symbol, base, quote, precision);
    let mut engine = state.engine.lock().unwrap();
    engine.create_market(market.clone());

    info!("Market created: {}", symbol);
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "message": format!("Market {} created", symbol),
        "market": {
            "symbol": market.symbol,
            "base_asset": market.base_asset,
            "quote_asset": market.quote_asset,
        }
    })))
}

#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/api/accounts",
    request_body = CreateAccountRequest,
    responses(
        (status = 200, description = "Account created successfully", body = CreateAccountResponse)
    )
))]
pub async fn create_account(
    state: web::Data<AppState>,
    req: web::Json<CreateAccountRequest>,
) -> ActixResult<HttpResponse> {
    let mut engine = state.engine.lock().unwrap();
    engine.create_account(req.account_id.clone());

    info!("Account created: {}", req.account_id);
    Ok(HttpResponse::Ok().json(CreateAccountResponse {
        success: true,
        message: format!("Account {} created", req.account_id),
    }))
}

#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/api/accounts/{account_id}/deposit",
    request_body = DepositRequest,
    responses(
        (status = 200, description = "Deposit successful", body = DepositResponse)
    )
))]
pub async fn deposit(
    state: web::Data<AppState>,
    account_id: web::Path<String>,
    req: web::Json<DepositRequest>,
) -> ActixResult<HttpResponse> {
    let mut engine = state.engine.lock().unwrap();

    match engine.deposit(account_id.to_string(), &req.asset, req.amount) {
        Ok(_) => {
            info!(
                "Deposit successful: {} {} to {}",
                req.amount, req.asset, account_id
            );
            Ok(HttpResponse::Ok().json(DepositResponse {
                success: true,
                message: format!("Deposited {} {} to {}", req.amount, req.asset, account_id),
            }))
        }
        Err(e) => {
            error!("Deposit failed: {:?}", e);
            Ok(HttpResponse::BadRequest().json(DepositResponse {
                success: false,
                message: format!("Deposit failed: {:?}", e),
            }))
        }
    }
}

#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/accounts/{account_id}/balance/{asset}",
    responses(
        (status = 200, description = "Balance retrieved", body = BalanceResponse)
    )
))]
pub async fn get_balance(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> ActixResult<HttpResponse> {
    let (account_id, asset) = path.into_inner();
    let engine = state.engine.lock().unwrap();
    let balance = engine.get_account_balance(&account_id, &asset);

    info!("Balance query: {} {}", account_id, asset);
    Ok(HttpResponse::Ok().json(BalanceResponse {
        account_id,
        asset,
        balance,
    }))
}

#[cfg_attr(feature = "swagger", utoipa::path(
    post,
    path = "/api/orders/place",
    request_body = PlaceOrderRequest,
    responses(
        (status = 200, description = "Order placed successfully", body = PlaceOrderResponse)
    )
))]
pub async fn place_order(
    state: web::Data<AppState>,
    req: web::Json<PlaceOrderRequest>,
) -> ActixResult<HttpResponse> {
    let side = match req.side.to_uppercase().as_str() {
        "BUY" => Side::Buy,
        "SELL" => Side::Sell,
        _ => {
            return Ok(HttpResponse::BadRequest().json(PlaceOrderResponse {
                success: false,
                message: "Invalid side. Use 'BUY' or 'SELL'".to_string(),
                trades: None,
            }))
        }
    };

    let order = Order::new(
        req.order_id,
        &req.account,
        &req.market,
        side,
        req.price,
        req.quantity,
        req.timestamp,
    );

    let mut engine = state.engine.lock().unwrap();
    match engine.place_limit_order(order) {
        Ok(trades) => {
            let trade_responses: Vec<TradeResponse> = trades
                .iter()
                .map(|t| TradeResponse {
                    trade_id: t.trade_id,
                    buy_order_id: t.buy_order_id,
                    sell_order_id: t.sell_order_id,
                    price: t.price,
                    qty: t.qty,
                })
                .collect();

            info!(
                "Order placed: id={}, account={}, market={}, side={}, qty={}",
                req.order_id, req.account, req.market, req.side, req.quantity
            );

            Ok(HttpResponse::Ok().json(PlaceOrderResponse {
                success: true,
                message: format!("Order placed. {} trades executed", trades.len()),
                trades: Some(trade_responses),
            }))
        }
        Err(e) => {
            error!("Order placement failed: {:?}", e);
            Ok(HttpResponse::BadRequest().json(PlaceOrderResponse {
                success: false,
                message: format!("Order placement failed: {:?}", e),
                trades: None,
            }))
        }
    }
}

#[cfg_attr(feature = "swagger", utoipa::path(
    get,
    path = "/api/positions/{account_id}/{market}",
    responses(
        (status = 200, description = "Position retrieved", body = PositionResponse),
        (status = 404, description = "Position not found", body = serde_json::Value)
    )
))]
pub async fn get_position(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> ActixResult<HttpResponse> {
    let (account_id, market) = path.into_inner();
    let engine = state.engine.lock().unwrap();

    match engine.get_position(&account_id, &market) {
        Some(position) => {
            info!("Position query: {} {}", account_id, market);
            Ok(HttpResponse::Ok().json(PositionResponse {
                account: position.account.clone(),
                market: position.market.clone(),
                quantity: position.quantity,
                avg_entry_price: position.avg_entry_price,
                realized_pnl: position.realized_pnl,
            }))
        }
        None => {
            info!("Position not found: {} {}", account_id, market);
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "success": false,
                "message": format!("Position not found for {} in {}", account_id, market)
            })))
        }
    }
}
