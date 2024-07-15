# Button - Simple User Input

1. Insert a html `button` into the view in `lib.rs`.
2. Log a message to console.
    ```rust
    #[component]
    pub fn App() -> impl IntoView {
        let name = "Bumblebee";
        let name_style = "color: blue;";
        view! {
            <h1>"Hello "<span class="name-label" style=name_style>{name}</span></h1>
            <button on:click=|_| { // (1)
                log::info!("Hello Button"); // (2)
            }>
                Click
            </button>
        }
    }
    ```
3. Adjust the `main.rs` to enable console logging.
    ```rust
    use leptos::*;
    use rust_leptos_live_coding::App;
    
    fn main() {
        _ = console_log::init_with_level(log::Level::Debug); // (3)
    
        mount_to_body(|| {
            view! {
                <App />
            }
        })
    }
    ```
