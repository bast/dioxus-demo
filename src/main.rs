use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    let mut count = use_state(&cx, || 0);
    let mut toggle_is_active = use_state(&cx, || true);

    use_future(&cx, (), move |_| {
        let mut count = count.clone();
        async move {
            loop {
                TimeoutFuture::new(3_000).await;
                count += 1;
            }
        }
    });

    cx.render(rsx! {
        h1 { "counter: {count}" }
        p { "toggle_is_active: {toggle_is_active}" }
        button { class: "uk-button uk-button-primary uk-margin-small-right",
                 onclick: move |_| toggle_is_active.set(!toggle_is_active), "toggle" }
        button { class: "uk-button uk-button-primary uk-margin-small-right",
                 onclick: move |_| count += 1, "more" }
        button { class: "uk-button uk-button-secondary uk-margin-small-right",
                 onclick: move |_| count -= 1, "less" }
        button { class: "uk-button uk-button-danger uk-margin-small-right",
                 onclick: move |_| count.set(0), "reset" }
    })
}
