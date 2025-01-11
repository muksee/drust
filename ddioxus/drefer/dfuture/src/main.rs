#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::Deserialize;

fn main() {
    dioxus_web::launch(App);
}

#[derive(Deserialize)]
struct ApiResponse {
    message: String,
    status: String,
}

fn App(cx: Scope) -> Element {
    let future = use_future(cx, (), |_| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<ApiResponse>()
            .await
    });

    cx.render(match future.value() {
        Some(Ok(response)) => rsx! {
            button {
                style: "height: 100px; width: 500px; font-size: 2em; border-radius: 0;",
                onclick: move |_| future.restart(),
                "Click to fetch another doggo"
            }
            div {
                img {
                    width: "500px",
                    max_height: "500px",
                    src: "{response.message}"
                }
            }
        },
        Some(Err(_)) => rsx! { div { "Loading dogs failed"}},
        None => rsx! { div { "Loading dogs "}},
    })
}
