# Hello World

1. Create `src/lib.rs`.
2. Add a `App` component:
   ```rust
   use leptos::*;
   
   #[component]
   pub fn App() -> impl IntoView {
        view! {
            <h1>"Hello World"</h1>
        }
   }
    ```
3. Create `src/main.rs`.
4. Mount the `App` component to DOM:
   ```rust
   use leptos::*;
   
   fn main() {
       mount_to_body(|| {
           view! {
               <App />
           }
       })
   }
   ```
5. Run trunk:
   ```shell
   trunk watch --open --release
   ```
