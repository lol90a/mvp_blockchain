MVP Blockchain Project
This project implements a decentralized blockchain-based application in Rust using WebAssembly (Wasm). The application includes the ability to add transactions to a blockchain and view the current state of the blockchain. The smart contract is written in Rust and compiled to WebAssembly, and a basic frontend is provided for interacting with the contract.

Features
Decentralization: The smart contract runs in a decentralized environment using WebAssembly.
Security: Transactions are managed securely through smart contracts.
Transparency: The blockchain can be viewed in real-time, showing all transactions.
Project Structure
bash
Copy code
mvp_blockchain/
├── app/                # Core logic of the blockchain smart contract
├── wasm/               # WebAssembly build files and Sails.idl generation
├── frontend/           # Frontend for interacting with the blockchain
├── Cargo.toml          # Rust project configuration
├── README.md           # Project documentation
app/
This folder contains the core logic for the blockchain, including definitions for transactions and blockchain events.

wasm/
The wasm/ folder contains the build setup for compiling the blockchain logic into WebAssembly, including the build.rs script for generating the Sails Interface Definition Language (IDL) file.
