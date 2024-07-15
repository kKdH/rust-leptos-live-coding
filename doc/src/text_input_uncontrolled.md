# Uncontrolled Text Input

1. Create a `NodeRef` for the input element.
2. Set the `node_ref` attribute of the input element.
3. Add a `button` to the view.
4. Use the `NodeRef` to get the input element's value.
5. Set the current name to the new value.
```rust
#[component]
pub fn App() -> impl IntoView {
    let names = vec!["Bumblebee", "Megatron", "Optimus", "Ironhide", "Soundwave"].into_iter().map(String::from).collect::<Vec<_>>();
    let current_name = create_rw_signal(names.last().cloned().expect("there should be at least one name"));
    let name_input: NodeRef<html::Input> = create_node_ref(); // (1)
    let style = String::from("color: blue;");
    view! {
        <Greeting name=current_name.read_only() style />
        <NameSwitch names current_name /><br/><br/>
        <input node_ref=name_input /> // (2)
        <button on:click=move |_| { // (3)
            let value = name_input.get() // (4)
                .expect("the input element should be mounted to DOM")
                .value();
            current_name.set(value); // (5)
        }>
            Say Hello
        </button>
    }
}
```
