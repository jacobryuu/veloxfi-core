# veloxfi-core サービス実装ガイド

## 概要

`veloxfi-core`は、VeloxFiのコア取引エンジンです。以下の機能を持つRESTful HTTPサービスとして動作します：

- **マーケット管理**: 複数の取引市場をサポート
- **アカウント管理**: ユーザーアカウント及び残高管理
- **オーダー管理**: 買い・売り注文の処理
- **ポジション管理**: ユーザーポジションの追跡
- **イベント永続化**: トレード履歴とアカウント変更の記録

---

## プロジェクト構造

```
veloxfi-core/
├── src/
│   ├── main.rs                 # エントリーポイント (Actix-webサーバー起動)
│   ├── lib.rs                  # ライブラリの公開インターフェース
│   │
│   ├── api/                    # REST API モジュール
│   │   ├── mod.rs              # APIサーバー構成・設定
│   │   └── handlers.rs         # APIエンドポイント実装
│   │
│   └── core/                   # コア取引エンジン
│       ├── mod.rs              # モジュール定義
│       ├── engine.rs           # 取引エンジン (オーダー・ポジション管理)
│       ├── market.rs           # マーケット定義
│       ├── order.rs            # オーダー データ構造
│       ├── orderbook.rs        # オーダーブック (マッチングエンジン)
│       ├── account.rs          # アカウント及び残高管理
│       ├── positions.rs        # ポジション管理
│       ├── events.rs           # イベント定義
│       └── persister.rs        # イベントログの永続化
│
├── Cargo.toml                  # 依存関係定義
├── API.md                      # API ドキュメント
└── SERVICE.md                  # このファイル

```

---

## 依存関係

### HTTPフレームワーク & 非同期ランタイム

| パッケージ | バージョン | 用途 |
|-----------|-----------|------|
| `actix-web` | 4.4 | HTTPサーバーフレームワーク |
| `actix-rt` | 2.9 | Actix-webのランタイム |
| `tokio` | 1.35 | 非同期ランタイム (full features) |

### データシリアライゼーション & ログ

| パッケージ | バージョン | 用途 |
|-----------|-----------|------|
| `serde` | 1.0 | シリアライゼーション フレームワーク |
| `serde_json` | 1.0 | JSON シリアライゼーション |
| `log` | 0.4 | ロギング インターフェース |
| `env_logger` | 0.11 | ロギング実装 |

### その他

| パッケージ | バージョン | 用途 |
|-----------|-----------|------|
| `thiserror` | 1.0 | エラー処理 |

---

## ビルド & 実行

### ビルド

```bash
# リリースビルド
cargo build --release

# デバッグビルド
cargo build
```

### 実行

```bash
# デフォルト (info ログレベル)
cargo run

# ログレベル指定
RUST_LOG=debug cargo run
RUST_LOG=info cargo run --release
```

サーバーは `http://127.0.0.1:8080` で起動します。

---

## APIエンドポイント

完全なAPIドキュメントは[API.md](./API.md)を参照してください。

### 主要エンドポイント

| メソッド | パス | 説明 |
|---------|------|------|
| `GET` | `/api/health` | ヘルスチェック |
| `POST` | `/api/markets` | マーケット作成 |
| `POST` | `/api/accounts` | アカウント作成 |
| `POST` | `/api/accounts/{id}/deposit` | 入金 |
| `GET` | `/api/accounts/{id}/balance/{asset}` | 残高確認 |
| `POST` | `/api/orders/place` | オーダー発注 |
| `GET` | `/api/positions/{id}/{market}` | ポジション確認 |

---

## アーキテクチャ設計

### レイヤー構成

```
┌─────────────────────────────────────────────────┐
│           REST API層 (HTTP)                      │
│     src/api/mod.rs + handlers.rs                 │
│     • リクエスト/レスポンス処理                   │
│     • JSON シリアライゼーション                   │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│         ビジネスロジック層                        │
│     src/core/engine.rs                           │
│     • オーダー処理                               │
│     • マッチングエンジン                         │
│     • ポジション更新                             │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│          データ管理層                            │
│     • Market (市場定義)                          │
│     • Order (オーダー)                           │
│     • Account (アカウント&残高)                  │
│     • Position (ポジション)                      │
└──────────────────┬──────────────────────────────┘
                   │
┌──────────────────▼──────────────────────────────┐
│        永続化層                                  │
│     src/core/persister.rs                        │
│     • events.log (イベントログ)                  │
│     • snapshot.json (スナップショット)           │
└─────────────────────────────────────────────────┘
```

### コンポーネント説明

#### API層 (`src/api/`)
- **mod.rs**: Actix-webサーバー起動、ルート定義
- **handlers.rs**: 各エンドポイントの実装

#### コア層 (`src/core/`)
- **engine.rs**: 取引エンジンの心臓。全取引ロジックを統合管理
- **orderbook.rs**: マッチングエンジン。買い・売りオーダーのマッチング処理
- **market.rs**: 市場の定義
- **order.rs**: オーダーデータ構造
- **account.rs**: アカウント & 残高管理
- **positions.rs**: ポジション追跡
- **events.rs**: イベント定義
- **persister.rs**: イベントログの読み書き

---

## 主な特徴

### 1. 確定的な取引処理
- オーダーマッチングは決定論的で再現可能
- イベントログから完全な取引履歴を復元可能

### 2. マルチユーザー対応
- 複数アカウント管理
- アカウント間の独立した残高&ポジション

### 3. 複数市場サポート
- 異なる通貨ペアの同時取引
- 市場ごとのオーダーブック

### 4. イベント駆動アーキテクチャ
- すべての状態変更をイベントとして記録
- リプレイ機能で任意の時点の状態を復元可能

### 5. スケーラビリティ
- HTTPサーバーで複数クライアントに対応
- Tokio非同期ランタイムで高スループット

---

## 使用フロー

### 1. サーバー起動
```bash
cargo run
```

### 2. マーケット作成
```bash
curl -X POST http://127.0.0.1:8080/api/markets \
  -H "Content-Type: application/json" \
  -d '{"symbol": "BTC-USD", "base": "BTC", "quote": "USD", "precision": 1}'
```

### 3. アカウント作成
```bash
curl -X POST http://127.0.0.1:8080/api/accounts \
  -H "Content-Type: application/json" \
  -d '{"account_id": "trader1"}'
```

### 4. 入金
```bash
curl -X POST http://127.0.0.1:8080/api/accounts/trader1/deposit \
  -H "Content-Type: application/json" \
  -d '{"account_id": "trader1", "asset": "USD", "amount": 1000000}'
```

### 5. オーダー発注
```bash
curl -X POST http://127.0.0.1:8080/api/orders/place \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": 1,
    "account": "trader1",
    "market": "BTC-USD",
    "side": "BUY",
    "price": 50000,
    "quantity": 1,
    "timestamp": 1234567890
  }'
```

### 6. ポジション確認
```bash
curl -X GET http://127.0.0.1:8080/api/positions/trader1/BTC-USD
```

---

## データ永続化

### イベントログ (`data/events.log`)
すべてのトレード関連イベントがJSONLフォーマットで記録されます：
```json
{"OrderPlaced":{"id":1,"account":"trader1","market":"BTC-USD",...}}
{"Trade":{"trade_id":1,"buy_order_id":1,"sell_order_id":2,...}}
{"Deposit":{"account_id":"trader1","asset":"USD","amount":1000000}}
```

### スナップショット (`data/snapshot.json`)
特定時点の完全な状態をJSONで保存可能：
```json
{
  "accounts": {...},
  "positions": {...},
  "orderbooks": {...}
}
```

---

## パフォーマンス最適化

### 既実装
- **BTreeMapベースのオーダーブック**: O(log n) のルックアップ
- **非同期I/O**: Tokioランタイムによる高スループット
- **イベント駆動**: 必要な時だけ処理を実行

### 今後の改善案
- メモリキャッシング
- データベース統合 (SQLite/PostgreSQL)
- キャッシュワーミング
- マッチングアルゴリズムの高度化

---

## トラブルシューティング

### ポート競合
```bash
# 別のポートで起動
PORT=8081 cargo run
```

### ログ出力が少ない
```bash
RUST_LOG=debug cargo run
```

### イベントログの削除
```bash
rm data/events.log data/snapshot.json
```

---

## ライセンス

MIT License

---

## サポート

問題が発生した場合は、GitHubのIssueを報告してください。
