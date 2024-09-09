
# MVP Blockchain with WebAssembly

This is an MVP (Minimum Viable Product) Blockchain project built using WebAssembly and Rust. The project includes a smart contract that manages basic blockchain transactions and a front-end interface to interact with it.

## Features

- **Decentralization**: The blockchain is managed by a smart contract deployed on a decentralized network.
- **Security**: Transactions are secure and stored immutably on the blockchain.
- **Transparency**: The blockchain is viewable, and all transactions are visible to the public.

## Project Structure

```
mvp_blockchain/
├── app/               # Rust-based smart contract source code
├── wasm/              # WebAssembly project files
│   ├── src/           # Source files for the WebAssembly contract
│   ├── build.rs       # Build script for generating IDL
│   └── Sails.idl      # Interface definition for smart contract
└── Cargo.toml         # Rust project dependencies and configuration
```

### Backend (Smart Contract)

- `app/`: Contains the Rust code for the blockchain smart contract.
- `wasm/`: Handles the WebAssembly build for the smart contract and includes the build script (`build.rs`).


## Prerequisites

Before you can build and run the project, ensure you have the following tools installed:

1. **Rust Toolchain** with WebAssembly target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

2. **Binaryen** (for WebAssembly optimization, optional):
   - Ubuntu/Debian:
     ```bash
     sudo apt install binaryen
     ```

## Build and Deploy

### 1. Clone the Repository

```bash
git clone https://github.com/lol90a/mvp_blockchain.git
cd mvp_blockchain
```

### 2. Build the Smart Contract

Navigate to the `wasm` directory and build the WebAssembly smart contract:

```bash
cd wasm
cargo build --target wasm32-unknown-unknown --release
```

This will produce a `.wasm` file in the `target/wasm32-unknown-unknown/release` directory.

###

## Project Files

- **`wasm/src/lib.rs`**: The main logic for the smart contract handling blockchain transactions.
- **`Sails.idl`**: Interface definition for the smart contract, describing the services and events.


---

## Troubleshooting

- If you encounter `wasm-opt` errors during the build, ensure it is installed and available in your `PATH`.
- If you're having issues with the IDL generation, check the enum and struct definitions in `wasm/src/lib.rs` to ensure they are well-formed.

---

## Contributing

Feel free to open issues and submit pull requests if you encounter bugs or have ideas for improvements.

---

## License

This project is licensed under the GPL-3.0 License 

---

## Acknowledgments

- [Gear Protocol](https://github.com/gear-tech/gear) for the tools and frameworks used in this project.
- [Vara-Lab Traffic Light Example](https://github.com/Vara-Lab/traffic-light-integration) for inspiration on integrating WebAssembly and Rust.

---
