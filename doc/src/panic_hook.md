# Panic Hook

1. Temporarily remove the call to the `cycle()` function:
    ```rust
    #[component]
    pub fn NameSwitch(names: Vec<&'static str>, current_name: RwSignal<&'static str>) -> impl IntoView {
        let mut infinite_names = names.into_iter()/*.cycle()*/; // (1)
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
2. Run the example and press the button multiple times, so the iterator returns `None` and the `expect` fails.
3. Inspect the error in the browser's log.
4. Configure the `console_error_panic_hook`:
    ```rust
    fn main() {
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once(); // (4)
    
        mount_to_body(|| {
            view! {
                <App />
            }
        })
    }
    ```
5. Run the example again and press the button multiple times, so the iterator returns `None` and the `expect` fails again.
6. Inspect the error in the browser's log again.
7. Insert the call to the `cycle()` function back again.
