# veloxfi-core HTTP API ドキュメント

## サービス起動

```bash
cargo run --release
```

サーバーは `http://127.0.0.1:8080` で起動します。

---

## API エンドポイント

### 1. ヘルスチェック
```
GET /api/health
```

**レスポンス:**
```json
{
  "status": "ok",
  "message": "veloxfi-core is running"
}
```

---

### 2. マーケット作成
```
POST /api/markets
```

**リクエスト:**
```json
{
  "symbol": "BTC-USD",
  "base": "BTC",
  "quote": "USD",
  "precision": 1
}
```

**レスポンス:**
```json
{
  "success": true,
  "message": "Market BTC-USD created",
  "market": {
    "symbol": "BTC-USD",
    "base_asset": "BTC",
    "quote_asset": "USD"
  }
}
```

---

### 3. アカウント作成
```
POST /api/accounts
```

**リクエスト:**
```json
{
  "account_id": "alice"
}
```

**レスポンス:**
```json
{
  "success": true,
  "message": "Account alice created"
}
```

---

### 4. 入金
```
POST /api/accounts/{account_id}/deposit
```

**リクエスト:**
```json
{
  "account_id": "alice",
  "asset": "USD",
  "amount": 100000000
}
```

**レスポンス:**
```json
{
  "success": true,
  "message": "Deposited 100000000 USD to alice"
}
```

---

### 5. 残高確認
```
GET /api/accounts/{account_id}/balance/{asset}
```

**例:**
```
GET /api/accounts/alice/balance/USD
```

**レスポンス:**
```json
{
  "account_id": "alice",
  "asset": "USD",
  "balance": 100000000
}
```

---

### 6. オーダー発注
```
POST /api/orders/place
```

**リクエスト:**
```json
{
  "order_id": 1,
  "account": "alice",
  "market": "BTC-USD",
  "side": "BUY",
  "price": 100000,
  "quantity": 1,
  "timestamp": 1234567890
}
```

**レスポンス:**
```json
{
  "success": true,
  "message": "Order placed. 1 trades executed",
  "trades": [
    {
      "trade_id": 1,
      "buy_order_id": 1,
      "sell_order_id": 2,
      "price": 100000,
      "qty": 1
    }
  ]
}
```

---

### 7. ポジション確認
```
GET /api/positions/{account_id}/{market}
```

**例:**
```
GET /api/positions/alice/BTC-USD
```

**レスポンス:**
```json
{
  "account": "alice",
  "market": "BTC-USD",
  "quantity": 1,
  "avg_entry_price": 100000,
  "realized_pnl": 0
}
```

---

## 使用例（curl）

### ヘルスチェック
```bash
curl -X GET http://127.0.0.1:8080/api/health
```

### マーケット作成
```bash
curl -X POST http://127.0.0.1:8080/api/markets \
  -H "Content-Type: application/json" \
  -d '{
    "symbol": "BTC-USD",
    "base": "BTC",
    "quote": "USD",
    "precision": 1
  }'
```

### アカウント作成
```bash
curl -X POST http://127.0.0.1:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"account_id": "alice"}'
```

### 入金
```bash
curl -X POST http://127.0.0.1:8080/api/accounts/alice/deposit \
  -H "Content-Type: application/json" \
  -d '{
    "account_id": "alice",
    "asset": "USD",
    "amount": 100000000
  }'
```

### 残高確認
```bash
curl -X GET http://127.0.0.1:8080/api/accounts/alice/balance/USD
```

### オーダー発注
```bash
curl -X POST http://127.0.0.1:8080/api/orders/place \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": 1,
    "account": "alice",
    "market": "BTC-USD",
    "side": "BUY",
    "price": 100000,
    "quantity": 1,
    "timestamp": 1234567890
  }'
```

### ポジション確認
```bash
curl -X GET http://127.0.0.1:8080/api/positions/alice/BTC-USD
```

---

## 依存関係

- `actix-web 4.4`: HTTPフレームワーク
- `tokio 1.35`: 非同期ランタイム
- `serde & serde_json`: JSON シリアライゼーション
- `log & env_logger`: ロギング

---

## 環境変数

ログレベルを指定:
```bash
RUST_LOG=info cargo run
RUST_LOG=debug cargo run
```

---

## アーキテクチャ

```
┌─────────────────────────────────────────────┐
│           HTTP クライアント                  │
└──────────────────┬──────────────────────────┘
                   │ HTTP/JSON
                   ▼
┌─────────────────────────────────────────────┐
│         Actix-web HTTP Server               │
│       (127.0.0.1:8080)                      │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│           API ハンドラー                    │
│      (src/api/handlers.rs)                  │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│      veloxfi-core エンジン                  │
│   ・マーケット管理                          │
│   ・オーダー管理                            │
│   ・ポジション管理                          │
│   ・イベント永続化                          │
└──────────────────┬──────────────────────────┘
                   │
                   ▼
┌─────────────────────────────────────────────┐
│           ファイルシステム                   │
│   data/events.log                           │
│   data/snapshot.json                        │
└─────────────────────────────────────────────┘
```
