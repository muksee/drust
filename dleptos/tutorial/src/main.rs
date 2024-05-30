#![allow(clippy::redundant_closure)]

use leptos::*;

fn main() {
    leptos::mount_to_body(|| leptos::view! { <App/>})
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let double_count = move || count();

    let html = "<p>This HTML will be injected.</p>";

    view! {
        <button
          on:click=move|_| { set_count.update(|n| *n +=1)}
          class:red=move || count()%2==1
        >
        "click me: " { count }
        </button>

        <p
        style="position: absolute"
        style:left=move || format!("{}px", count() + 100)
        style:background-color=move || format!("rgb({}, {}, 100)", count(), 100)
        style:max-width="400px"
        style=("--columns", count)
        >
        <strong>"Reactive: "</strong>
        { move || count()}
        </p>
        <p>
        <strong>"Reactive shorthand: "</strong>
        { count }
        </p>
        <p>
        <strong>"Not reactive: "</strong>
        { count() }
        </p>

        <p>
        <strong>"Double Count:" </strong>
        { double_count },{ double_count }
        </p>

        <p inner_html=html/>

        <p>
        <h3>"Progress bar"</h3>
        <progress max="50" value=count>
        </progress>
        <br/>
        <progress max="50" value=double_count>
        </progress>
        </p>

        <p>
        <h3>"Progress bar"</h3>
        <ProgressBar max="50" progress=count>
        </ProgressBar>
        </p>
    }
}

#[component]
fn progress_bar(max: &'static str, progress: ReadSignal<i32>) -> impl IntoView {
    view! {
        <progress
            max=max
            // now this works
            value=progress
        />
    }
}
