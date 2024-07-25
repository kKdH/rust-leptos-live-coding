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

## Serve web-app

Run the following command from the repositories root:

```shell
trunk serve --release --open
```

## Serve book

Navigate into the `doc` directory and run:

```shell
mdbook serve --open
```

## Useful resources

- Leptos home page: https://leptos.dev/
- Leptos book: https://book.leptos.dev/
- Trunk: https://trunkrs.dev
