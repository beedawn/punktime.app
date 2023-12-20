use leptos::*;

fn main() {
   mount_to_body(|| view!{ <App /> });





}



#[component]
fn App() -> impl IntoView {


  let (auth, set_auth) = create_signal(false);
    view!{
      <div>

 <Show
    when=move || auth.get() == false
    fallback=|| view! {  <LoggedIn />}
  >
   <div style="
    width:350px;
    height:550px;
    padding:100px;
      margin:auto;

      ">

<h1 style="font-family:sans-serif;">punktime</h1>
      <Login auth=auth set_auth=set_auth />
 </div>
  </Show>
         </div>   }


}



#[component]
fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>) -> impl IntoView {


let (name, set_name) = create_signal(String::new());
let (pw, set_pw) = create_signal(String::new());
view!{

  <div style="height:125px;font-family:sans-serif;font-size:14px;padding:25px;border-width:5px;border:solid;">
   <div style="padding:10px 15px">Username:  <input type="text" on:input=move |ev| { set_name(event_target_value(&ev));} prop:value=name /></div>
    
   <div style="padding:10px 15px;">Password:  <input type="password" on:input=move |ev| {set_pw(event_target_value(&ev));} prop:value=pw /></div>
   <div style="position:absolute;padding:10px 20px;left:40%;"> 
    <button on:click= move |_| {

  set_auth(!auth.get());

  //send credentials to server for validation


  }> Login </button>
</div>
  </div>
}

}




#[component]
fn LoggedIn()-> impl IntoView{
view!{"Logged in"}
}
