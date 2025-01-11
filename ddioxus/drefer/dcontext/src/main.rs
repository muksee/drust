#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    // 组件共享
    // dioxus_web::launch(MemeEditor);
    // 全局共享
    dioxus_web::launch(GlobalStateApp);
}

fn MemeEditor(cx: Scope) -> Element {
    let container_style = r"
    display: flex;
    flex-direction: column;
    gap: 16px;
    margin: 0 auto;
    width: fit-content;
    ";
    let caption =
        use_state(cx, || "me waiting for my rust code to compile".to_string());

    cx.render(rsx!(
        div {
            style: "{container_style}",
            h1 { "Meme Editor"},
            Meme {
                caption: caption,
            },
            CaptionEditor{
                caption: caption,
                on_input: move |event: FormEvent| { caption.set(event.value.clone());},
            }
        }
    ))
}

#[inline_props]
fn Meme<'a>(cx: Scope<'a>, caption: &'a str) -> Element<'a> {
    let container_style = r#"
    position: relative;
    width: fit-content;
    "#;

    let caption_container_style = r#"
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    padding: 16px 8px;
    "#;

    let caption_style = r"
    font-size: 32px;
    margin: 0;
    color: red;
    text-align: center;
    ";

    cx.render(rsx!(div {
        style: "{container_style}",
        img {
            src: "https://img.ixintu.com/download/jpg/202001/7f9004670cd193a69998db0ea1b53ca8.jpg!con",
            height: "500px",
                },
        div {
            style: "{caption_container_style}",
            p {
                style: "{caption_style}",
                "{caption}"
            }
        }
    }))
}

#[inline_props]
fn CaptionEditor<'a>(
    cx: Scope<'a>,
    caption: &'a str,
    on_input: EventHandler<'a, FormEvent>,
) -> Element<'a> {
    let input_style = r"
    border: none;
    background: cornflowerblue;
    padding: 8px 16px;
    margin: 0;
    border-radius: 4px;
    color: white;
    ";

    cx.render(rsx!(input {
        style: "{input_style}",
        value: "{caption}",
        oninput: move |event| on_input.call(event),
    }))
}

fn GlobalStateApp(cx: Scope) -> Element {
    struct DarkMode(bool);
    use_shared_state_provider(cx, || DarkMode(false));

    let dark_mode = use_shared_state::<DarkMode>(cx).unwrap();

    let style = if dark_mode
        .read()
        .0
    {
        r"
        color: white; 
        background-color: black;
        padding: 10px; margin: 10px
        "
    } else {
        r"
        padding: 10px; margin: 10px
        border: 1px solid black;
        "
    };

    cx.render(rsx!(label {
        style: "{style}",
        "dark mode",
        input {
            r#type: "checkbox",
            oninput: move |event| {
                let is_enabled = event.value == "true";
                dark_mode.write().0 = is_enabled;
            }
        }
    }))
}
