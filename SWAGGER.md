# Swagger/OpenAPI ドキュメント

## 概要

veloxfi-core には、デバッグビルド時に **Swagger UI** でAPI仕様を閲覧・テストできる機能があります。

---

## ビルドモード

### デバッグビルド（Swagger有効）✅
```bash
cargo build           # または
cargo run
```

**特徴:**
- Swagger UIが有効
- OpenAPI仕様が自動生成
- 詳細なデバッグ情報が含まれる
- バイナリサイズ: ~21MB

### リリースビルド（Swagger無効）
```bash
cargo build --release --no-default-features
./target/release/veloxfi-core
```

**特徴:**
- Swagger UIは無効（依存関係なし）
- 最適化されたバイナリ
- 本番環境対応
- バイナリサイズ: ~5.9MB（72%削減）

---

## Swagger UIへのアクセス

### 1. デバッグビルドで実行
```bash
cargo run
```

サーバーが起動したら、ブラウザで以下にアクセス：

```
http://127.0.0.1:8080/swagger-ui/
```

### 2. Swagger UIの画面

**表示内容:**
- すべてのAPIエンドポイント一覧
- リクエスト/レスポンスのスキーマ
- パラメータの説明
- サンプルリクエスト

### 3. API仕様を直接確認

```bash
curl http://127.0.0.1:8080/api-docs/openapi.json | jq .
```

---

## Swagger UIでのテスト方法

### 1. エンドポイントをクリック
Swagger UIでエンドポイント（例：`POST /api/accounts`）をクリック

### 2. 「Try it out」をクリック
```
[Try it out]
```

### 3. リクエストボディを入力
```json
{
  "account_id": "test_user"
}
```

### 4. 「Execute」をクリック
```
[Execute]
```

### 5. レスポンスを確認
```json
{
  "success": true,
  "message": "Account test_user created"
}
```

---

## 機能別 API ドキュメント

### マーケット管理

#### POST /api/markets
市場を作成

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

### アカウント管理

#### POST /api/accounts
アカウント作成

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

#### POST /api/accounts/{account_id}/deposit
資金入金

**リクエスト:**
```json
{
  "account_id": "alice",
  "asset": "USD",
  "amount": 1000000
}
```

**レスポンス:**
```json
{
  "success": true,
  "message": "Deposited 1000000 USD to alice"
}
```

#### GET /api/accounts/{account_id}/balance/{asset}
残高確認

**パラメータ:**
- `account_id`: alice
- `asset`: USD

**レスポンス:**
```json
{
  "account_id": "alice",
  "asset": "USD",
  "balance": 1000000
}
```

---

### オーダー管理

#### POST /api/orders/place
オーダー発注

**リクエスト:**
```json
{
  "order_id": 1,
  "account": "alice",
  "market": "BTC-USD",
  "side": "BUY",
  "price": 50000,
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
      "price": 50000,
      "qty": 1
    }
  ]
}
```

---

### ポジション管理

#### GET /api/positions/{account_id}/{market}
ポジション確認

**パラメータ:**
- `account_id`: alice
- `market`: BTC-USD

**レスポンス:**
```json
{
  "account": "alice",
  "market": "BTC-USD",
  "quantity": 1,
  "avg_entry_price": 50000,
  "realized_pnl": 0
}
```

---

## OpenAPI 仕様

### OpenAPI バージョン
```
3.0.0
```

### API情報
```yaml
title: veloxfi-core
version: 0.1.0
```

### サーバー
```
http://127.0.0.1:8080
```

### 自動生成されるエンドポイント
```
GET     /api/health
POST    /api/markets
POST    /api/accounts
POST    /api/accounts/{account_id}/deposit
GET     /api/accounts/{account_id}/balance/{asset}
POST    /api/orders/place
GET     /api/positions/{account_id}/{market}
```

---

## Swagger UI で利用可能な機能

✅ **API エクスプローラー**
- すべてのエンドポイントを一覧表示
- パラメータの詳細情報を表示

✅ **インタラクティブテスト**
- ブラウザから直接APIをテスト
- リクエストを編集してExecute

✅ **スキーマ表示**
- リクエスト/レスポンスボディの構造を表示
- 各フィールドの説明を表示

✅ **自動ドキュメント生成**
- コード変更時に自動更新
- 常に最新の仕様を表示

---

## 使用例

### curl で OpenAPI スキーマを取得
```bash
curl http://127.0.0.1:8080/api-docs/openapi.json > openapi.json

# スキーマを見やすく表示
cat openapi.json | jq .
```

### Swagger コマンドラインツールで生成
```bash
# Swagger CLI がインストールされている場合
swagger validate http://127.0.0.1:8080/api-docs/openapi.json
```

### Postman にインポート
1. Swagger UI にアクセス
2. `/api-docs/openapi.json` をコピー
3. Postman で「Import」> 「Paste Raw Text」
4. OpenAPI スキーマをインポート

---

## トラブルシューティング

### Swagger UI が表示されない

**症状:** `http://127.0.0.1:8080/swagger-ui/` にアクセスできない

**原因:** リリースビルドで実行している

**解決策:**
```bash
# デバッグビルドで実行
cargo build
cargo run
```

### OpenAPI スキーマが空

**症状:** OpenAPI スキーマ（`/api-docs/openapi.json`）の内容が不完全

**原因:** `#[utoipa::path]` マクロが正しく適用されていない

**確認方法:**
```bash
# ハンドラーに以下が付いているか確認
#[cfg_attr(feature = "swagger", utoipa::path(...))]
```

### リリースビルドのサイズが大きい

**症状:** `--no-default-features` オプションを使用しても大きい

**原因:** デバッグシンボルが含まれている

**解決策:**
```bash
# プロファイル設定を使用
cargo build --release --no-default-features
```

---

## ビルド最適化

### デバッグビルド最適化
```bash
# Swagger を除外したデバッグビルド
cargo build --no-default-features
```

### リリースビルド最適化
```bash
# 本番環境用（Swagger なし）
cargo build --release --no-default-features

# ストリップ（シンボル削除）
strip target/release/veloxfi-core
```

---

## セキュリティに関する注意

⚠️ **重要:** 本番環境では Swagger UI を有効にしないでください

**理由:**
- APIスキーマが公開されると、攻撃者に詳細な情報を与える
- 不必要なバイナリサイズの増加

**推奨:**
- 本番環境: `--no-default-features` でビルド
- 開発環境: デフォルト（Swagger有効）でビルド
- ステージング: 必要に応じて有効化

---

## 今後の拡張

- [ ] SwaggerUI の認証サポート
- [ ] レート制限の表示
- [ ] エラーレスポンスの詳細化
- [ ] Webhook定義の追加
