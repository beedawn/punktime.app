#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        body{
        background_color:"yellow",
        height:"550px",
        width:100,
        div 
        {
            background_color:"green",
            h1{"Welcome to Punktime!"}
        }
        div{
            p{"Testing"}
        }}
    })
}
