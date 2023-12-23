use leptos::*;

mod apirequest;
mod navbar;

#[component]
pub fn Dashboard(auth:ReadSignal<bool>,set_auth:WriteSignal<bool>)-> impl IntoView{
view!{
    <div>
        <navbar::NavBar auth=auth set_auth=set_auth />
        <apirequest::APIRequest /> 
    </div>
    }
}


