# Signales

1. Create a `Vec` of names.
2. Create a RwSignal to change the current displayed name.

`lib.rs`
```rust
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let names = vec!["Bumblebee", "Megatron", "Optimus", "Ironhide", "Soundwave"]; // (1)
    let current_name = create_rw_signal(names.last().cloned().expect("there should be at least one name")); // (2)
    let name_style = String::from("color: blue;");
    view! {
        <Greeting name=current_name.read_only() name_style />
        <NameSwitch names current_name />
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Greeting(name: ReadSignal<&'static str>, name_style: String) -> impl IntoView {
    view! {
        <h1>"Hello "<span class="name-label" style=name_style>{name}</span></h1>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn NameSwitch(names: Vec<&'static str>, current_name: RwSignal<&'static str>) -> impl IntoView {
    let mut infinite_names = names.into_iter().cycle();
    view! {
        <button on:click=move |_| {
            let next_name = infinite_names.next().expect("the iterator should be infinite");
            log::info!("Next: {next_name}");
            current_name.set(next_name)
        }>
            Next Name
        </button>
    }
}

```
