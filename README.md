# UOMI Helper Agent — Wasp + Rust (WASM)

This repository is a **Wasp** project that includes a Rust-based agent compiled to **WebAssembly (WASM)**.

## Included
- `main.wasp` — Wasp app configuration
- `wasp.config.json` — basic project metadata
- `package.json` — npm metadata (placeholder)
- `agent/` — Rust crate for the agent
  - `agent/src/lib.rs` — Rust agent code exposing `handle_input` for WASM host
  - `agent/Cargo.toml` — crate config

## Build (developer / laptop)
1. Install Rust and add WASM target:
   ```bash
   rustup target add wasm32-unknown-unknown
   ```
2. Build the Rust crate to WASM:
   ```bash
   cd agent
   cargo build --target wasm32-unknown-unknown --release
   ```
3. The compiled `.wasm` will be at:
   `target/wasm32-unknown-unknown/release/agent.wasm`
4. Use Wasp tooling / host to run the project as needed.

## Agent features (demo)
- Greeting, UOMI info (roles, token, validators)
- Simulated token send with validation and confirmation for large amounts
- Simple rate limiting (per-process counter)
- Clear textual responses (emoji-friendly)
