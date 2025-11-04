# 🪙 MiniCoin – A Simple Blockchain in Rust

This is a **beginner-friendly blockchain simulation** written in Rust.  
It demonstrates key Rust concepts such as **structs, enums, vectors, ownership, and borrowing**, while modeling a simplified version of how cryptocurrency ledgers work.

---

## 🚀 Overview

The project mimics a blockchain ledger:

- **Transactions** represent the transfer of funds between users.
- **Blocks** store a list of transactions and link to the previous block via a "hash".
- **The Blockchain** is a collection (`Vec`) of all blocks, maintaining the history of all transfers.

While this doesn’t include cryptography or proof-of-work, it’s a great foundation for understanding how blockchains store and verify data immutably.

---

## 🧱 Core Components

### 1. `Transaction` Struct
Stores transaction details:
```rust
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}
