# Controlled Text Input

1. Add an `input` element into the view.
2. The `event_target_value` is of type `String` so change all spots where a name is used from `&'static str` to `String`.

```rust
use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let names = vec!["Bumblebee", "Megatron", "Optimus", "Ironhide", "Soundwave"].into_iter().map(String::from).collect::<Vec<_>>(); // (2)
    let current_name = create_rw_signal(names.last().cloned().expect("there should be at least one name"));
    let style = String::from("color: blue;");
    view! {
        <Greeting name=current_name.read_only() style />
        <NameSwitch names current_name /><br/><br/>
        <input on:input=move |event| { // (1)
            let value = event_target_value(&event);
            current_name.set(value);
        } />
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Greeting(name: ReadSignal<String>, style: String) -> impl IntoView { // (2)
    view! {
        <h1>"Hello "<span class="name-label" style=style>{name}</span></h1>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn NameSwitch(names: Vec<String>, current_name: RwSignal<String>) -> impl IntoView { // (2)
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
