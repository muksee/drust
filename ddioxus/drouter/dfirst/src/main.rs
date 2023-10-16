#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;

fn main() {
    dioxus_web::launch(App)
}

fn App(cx: Scope) -> Element {
    render! {
        Router::<Route>{}
    }
}

#[inline_props]
fn NavBar(cx: Scope) -> Element {
    render! {
        nav {
            ul {
                li {
                    span {
                        style: "padding: 10px;",
                        Link {
                            to: Route::Home {  },
                            "Home"
                        }
                    }
                    span {
                        style: "padding: 10px;",
                        Link {
                            to: Route::BlogList {  },
                            "Blog"
                        }
                    }
                }
            }
        }
        hr {}
        Outlet::<Route>{}
    }
}

#[inline_props]
fn Home(cx: Scope) -> Element {
    render! {
        h1 { "Welcom to the dioxus HOME"}
    }
}

#[inline_props]
fn Blog(cx: Scope) -> Element {
    render! {
        h1 { "BLOG"}
        Outlet::<Route>{}
    }
}

#[inline_props]
fn BlogList(cx: Scope) -> Element {
    render! {
        h2 { "choose a post"}
        ul {
            li {
                Link {
                    to: Route::BlogPost { name: "Blog post 1".into() },
                    "Read the first blog post"
                }
            }
            li {
                Link {
                    to: Route::BlogPost { name: "Blog post 2".into() },
                    "Read the second blog post"
                }
            }
        }
    }
}

#[inline_props]
fn BlogPost(cx: Scope, name: String) -> Element {
    render! {
        h2 { "Blog Post: {name}"}
    }
}

#[inline_props]
fn PageNotFound(cx: Scope, segments: Vec<String>) -> Element {
    render! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre {
            color: "red",
            "log:\nattemped to navigate to: {segments:?}"
        }
    }
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/")]
        Home {},
        #[nest("/blog")]
            #[layout(Blog)]
                #[route("/")]
                BlogList {},
                #[route("/post/:name")]
                BlogPost { name: String },
            #[end_layout]
        #[end_nest]
    #[end_layout]
    #[route("/:..segments")]
    PageNotFound { segments: Vec<String> },
}
