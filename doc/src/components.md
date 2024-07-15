# Components

1. Implement a new leptos components in `src/lib.rs` for the name label and button.
2. Replace the heading with the new component and pass the parameters into it.
3. Replace the button with the new button component.

`lib.rs`
```rust
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let name = String::from("Elmar");
    let style = String::from("color: blue;");
    view! {
        <Greeting name style /> // (2)
        <Button /> // (3)
    }
}

#[component]
pub fn Greeting(name: String, style: String) -> impl IntoView { // (1)
    view! {
        <h1>"Hello "<span class="name-label" style=style>{name}</span></h1>
    }
}

#[component]
pub fn Button() -> impl IntoView { // (1)
    view! {
        <button on:click=|_| {
            log::info!("Hello Button");
        }>
            Click
        </button>
    }
}
```
