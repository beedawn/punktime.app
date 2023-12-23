use leptos::*;
mod login;
mod dashboard;

#[component]
pub fn App() -> impl IntoView {
  let (auth, set_auth) = create_signal(false);
    view!{
        <div>
            <Show
            when=move || auth.get() == false
            fallback=move || view! {  
                <dashboard::Dashboard auth=auth set_auth=set_auth />
            }
            >
                <div style="
                width:350px;
                height:550px;
                padding:100px;
                margin:auto;
                ">
                    <h1 style="font-family:sans-serif;">
                    punktime
                    </h1>
                    <login::Login auth=auth set_auth=set_auth />
                </div>
            </Show>
        </div>   
    }
}


