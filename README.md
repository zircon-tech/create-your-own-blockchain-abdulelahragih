## RustyChain 

This project is a simple **educational blockchain** implementation in Rust, designed for demonstration purposes. It allows you to create and validate blocks with transactions, simulate Proof-of-Work mining, and persist the blockchain state to a JSON file.

---

### Features
- **Transactions** with `from`, `to`, and `amount`
- **Proof-of-Work** with adjustable difficulty
- **Blockchain validation**
- **Persistent storage** to `blockchain.json` (for simplicity)
- **Interactive CLI** for adding transactions

---

### How It Works

Each time the program runs:

1. It loads the blockchain from `blockchain.json` if it exists.
2. Prompts the user to enter transaction details.
3. Creates a new block containing the transaction.
4. Mines the block using a basic Proof-of-Work mechanism (hash must start with `0000`).
5. Adds the block to the chain and saves it back to disk.

---

### Prerequisites
- Install Rust and Cargo

### Getting Started

#### 1. Clone the repository

#### 2. Build the project and install dependencies
```bash
cargo build
```
#### 3. Run the project
```bash
cargo run
```

---

### Example Interaction

```txt
--- New Transaction ---
From: Abdulelah
To: Sarah
Amount: 10
✅ Block added and mined.
Blockchain valid? true
```

---

### Files

- `main.rs` – main blockchain logic and CLI.
- `blockchain.json` – saved blockchain state.