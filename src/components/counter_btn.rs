use leptos::*;

#[component]
pub fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let count = create_rw_signal(0);
    view! {
        <button
            on:click= move |_| {
                count.update(|value| *value = *value + increment);
                if count.get_untracked() > 5 {
                    panic!("Boom");
                }
            }
        >
            "Click me: " {count}
        </button>
    }
}
