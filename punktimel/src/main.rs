use leptos::*;

fn main() {
   mount_to_body(|| view!{ <App /> });





}



#[component]
fn App() -> impl IntoView {


  let (auth, set_auth) = create_signal(false);
    view!{
 <Show
    when=move || auth.get() == false
    fallback=|| view! {  <LoggedIn />}
  >
   <div style="position:absolute;
    top:40%;
    left:42%;

      "><Login auth=auth set_auth=set_auth />
 </div>
  </Show>
            }


}



#[component]
fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>) -> impl IntoView {
let (name, set_name) = create_signal("Controlled".to_string());
let (pw, set_pw) = create_signal("spaghetti2".to_string());
view!{
   <div>Username:  <input type="text" on:input=move |ev| { set_name(event_target_value(&ev));} prop:value=name /></div>
    
   <div>Password:  <input type="text" on:input=move |ev| {set_pw(event_target_value(&ev));} prop:value=pw /></div>
    
    <button on:click= move |_| {

  set_auth(!auth.get());

  //send credentials to server for validation


  }> Login </button>
}

}




#[component]
fn LoggedIn()-> impl IntoView{
view!{"Logged in"}
}
