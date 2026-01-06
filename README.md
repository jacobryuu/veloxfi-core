# veloxfi-core

## Overview

**veloxfi-core** is the off-chain core trading system of VeloxFi.

It is responsible for **all trading-related logic**, including:

* Order processing
* Position management
* Risk control
* Internal ledger

This system operates **independently of blockchain** and does not require smart contracts to function.

---

## Core Responsibilities

veloxfi-core is the **single source of truth** for trading state.

* Fast, deterministic execution
* Recoverable state
* Regulation-friendly design
* Blockchain-agnostic

---

## Main Functional Domains

### 1. Market & Price Module

**Responsibilities**

* Market definition (FX, Crypto)
* Symbol metadata
* Price feed ingestion

**Functions**

* Register markets (e.g. EUR/USD, BTC/USDT)
* Normalize external price feeds
* Provide latest price snapshots

**Notes**

* External price sources are abstracted
* Mock price providers must be supported

---

### 2. Order Management

**Responsibilities**

* Accept and validate orders
* Maintain order lifecycle

**Functions**

* Place order (market / limit)
* Cancel order
* Order state tracking (NEW, FILLED, CANCELLED)

**Notes**

* No on-chain interaction
* Orders are internal objects only

---

### 3. Matching Engine

**Responsibilities**

* Match buy/sell orders
* Generate trades deterministically

**Functions**

* Order book management
* Trade generation
* Partial / full fill handling

**Notes**

* Deterministic behavior required
* Replayable from event log

---

### 4. Position Management

**Responsibilities**

* Track user positions
* Maintain average price and size

**Functions**

* Open / increase / reduce positions
* Close positions
* Position snapshot query

**Notes**

* FX and Crypto share unified position model
* Asset precision handled internally

---

### 5. PnL & Margin Calculation

**Responsibilities**

* Calculate unrealized / realized PnL
* Enforce margin rules

**Functions**

* Margin requirement calculation
* Liquidation threshold detection
* PnL snapshot generation

**Notes**

* Uses abstract price interface
* Does not trigger settlement directly

---

### 6. Risk Management

**Responsibilities**

* Prevent invalid trades
* Protect system solvency

**Functions**

* Pre-trade risk checks
* Exposure limits
* Account-level risk flags

**Notes**

* Must fail fast
* No auto-recovery logic

---

### 7. Internal Ledger

**Responsibilities**

* Track balances and transfers
* Act as accounting backbone

**Functions**

* Credit / debit balances
* Freeze / unfreeze funds
* Ledger snapshot export

**Notes**

* Ledger is append-only
* Source of truth for settlement

---

### 8. Settlement Interface (Optional)

**Responsibilities**

* Prepare settlement data
* Interact with veloxfi-contracts (if enabled)

**Functions**

* Generate settlement instructions
* Export state hash
* Settlement status tracking

**Notes**

* Entirely optional
* Core trading works without this module

---

### 9. User & Account Domain (Minimal Core)

**Responsibilities**

* Maintain trading identity
* Associate balances and positions

**Functions**

* Account creation
* Account status (ACTIVE / SUSPENDED)
* Account-level limits

**Notes**

* No KYC / Auth logic here
* Integrated via external services

---

## Cross-Cutting Concerns

### Event System

* All state changes emit internal events
* Used for replay, audit, recovery

### Persistence

* Write-ahead logging
* Snapshot + replay recovery

### Extensibility

* FX / Crypto via common interfaces
* New asset classes pluggable

---

## Explicit Non-Goals

veloxfi-core will NOT:

* Perform blockchain transactions
* Manage private keys
* Authenticate users
* Handle UI or APIs directly

---

## Key Principle

> **veloxfi-core is the engine.
> Everything else is replaceable.**
