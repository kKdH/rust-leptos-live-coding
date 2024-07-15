# Project Setup

1. Create `Cargo.toml`
   ```toml
   [package]
   name = "rust-leptos-live-coding"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   leptos = { version = "0.6", features = ["csr"] }
   leptos_meta = { version = "0.6", features = ["csr"] }
   leptos_router = { version = "0.6", features = ["csr"] }
   console_log = "1"
   log = "0.4"
   console_error_panic_hook = "0.1"
   
   [profile.release]
   opt-level = 'z'
   lto = true
   codegen-units = 1
   panic = "abort"
   ```
