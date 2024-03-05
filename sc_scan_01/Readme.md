## Sc scan 01

Inspect wasm files

# Organisation

* src:
  * Wasm library code
* tools:
  * wasm_test_01:
    * a basic wasm using Massa libs (for unit testing)
  * wasm_test_02:
    * a basic wasm using Rust (for unit testing)
  * demonstrator_www:
    * a very basic web server
      * user can input a smart contract address and view bytecode parse results

# Run the demo www site

## Compile wasm

* cargo install wasm-pack
* wasm-pack build --target web
* cp -v pkg/sc_scan_01_bg.wasm tools/demonstrator_www/assets/
* cp -v pkg/*.js tools/demonstrator_www/assets/

## Run

* cd tools/demonstrator_www
* EDIT assets/myjs.js (line 6) WALLET_SECRET_KEY
* RUST_LOG=debug cargo run
  * Use Firefox to go to url: http://127.0.0.1:3000/
  * Enter a Smart Contract address
    * massa sc examples hello world:
      * AS12VNrxjEzyFJMFjfmLmzt7Wf8MWiaiX829KRGEzDR2C9GR5B3bq
    * massa sc examples age:
      * AS12R7SqhV5LYww8eFjkMd5rBRqEwXs86i5TEHeRywaydttkmaL2R
    * massa sc examples blog:
      * AS12wfRwPmRzWCX9PQVKUdWD2Fbu91PfmEbL4KnHqLEjSz6fqJgah
  * View the result
  * Use developer tools to view additional console.log

# Dev

## run unit tests

### Setup

* cargo install wasm-pack
* cd tools/wasm_test_01 && npm init && npm build_test_debug
* cd tools/wasm_test_02 && wasm-pack build
* cargo clean
* wasm-pack test --node

# QA

* cargo fmt
* cargo clippy