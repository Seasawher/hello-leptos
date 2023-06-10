use leptos::*;

fn main() {
    mount_to_body(|cx| {
        view! {cx, <App/> }
    })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! {cx,
        <button
            on:click=move |_| {
                set_count.update(|n| *n += 1);
            }
            class:red=move || count() % 2 == 1
        >
        "Click me: "
        {move || count.get()}
        </button>

        <ProgressBar progress=count />
    }
}

#[component]
fn ProgressBar(cx: Scope, progress: ReadSignal<i32>) -> impl IntoView {
    view! {cx,
        <progress
            max="50"
            value=progress
        />
    }
}
