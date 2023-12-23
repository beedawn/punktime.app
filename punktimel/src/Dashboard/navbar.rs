use leptos::*;
#[component]
pub fn NavBar(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>)-> impl IntoView{

    view!{
        <div>
            <button
            on:click= move |_| {
            //flips auth state
            set_auth.set(!auth.get());
            }
            >Log Out</button>
        </div>
    }
}