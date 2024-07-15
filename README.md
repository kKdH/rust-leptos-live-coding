# Rust Leptos Live Coding

## Setup

1. Install the Rust's WASM target.
    ```shell
    rustup target install wasm32-unknown-unknown
    ```

2. Install Trunk:
    ```shell
    cargo install --locked trunk
    ```

## Serve

```shell
trunk serve --release --open
```
