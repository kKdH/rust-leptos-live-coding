# Rust Leptos Live Coding

## Setup

1. Install the Rust's WASM target.
    ```shell
    rustup target install wasm32-unknown-unknown
    ```

2. Install Trunk:
    ```shell
    cargo install trunk
    ```

3. Install mbBook
   ```shell
      cargo install mdbook
   ```

## Serve

```shell
trunk serve --release --open
```
