#![allow(non_snake_case)]
// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    // launch the dioxus app in a webview
    dioxus_desktop::launch(App);
}

// define a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element 

{


     let pw = use_state(cx, || "".to_string());
     let name = use_state(cx, || "".to_string());
    cx.render(rsx! {
        body{
        background_color:"yellow",
        height:"550px",
        width:100,
        div 
        {
            background_color:"green",
            h1{"Welcome to Punktime!"}
            h2{"{name}"}
            h2{"{pw}"}
        }
        div{
            p{"Testing"}
           div{"Username: " input{
            value: "{name}",
            oninput: move|evt| name.set(evt.value.clone()),


                }
           }

           div {

            "Password: "
                input{
                    value:"{pw}",
                    oninput: move |evt| pw.set(evt.value.clone()),
                }

           }



        }}
    })
}
