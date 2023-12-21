use leptos::*;
use reqwasm::http::Request;
use reqwasm::http::ReadableStream;

fn main() {
   mount_to_body(|| view!{ <App /> });





}



#[component]
fn App() -> impl IntoView {


  let (auth, set_auth) = create_signal(false);
  let (response_string, set_response_string) = create_signal(String::new());
    view!{
      <div>

 <Show
    when=move || auth.get() == false
    fallback=move || view! {  <LoggedIn auth=auth set_auth=set_auth response_string=response_string set_response_string=set_response_string />}
  >
   <div style="
    width:350px;
    height:550px;
    padding:100px;
      margin:auto;

      ">

<h1 style="font-family:sans-serif;">punktime</h1>
      <Login auth=auth set_auth=set_auth response_string=response_string set_response_string=set_response_string  />
 </div>
  </Show>
         </div>   }


}



#[component]
fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>, response_string:ReadSignal<String>, set_response_string:WriteSignal<String>) -> impl IntoView {
println!("Hello");

let (name, set_name) = create_signal(String::new());
let (pw, set_pw) = create_signal(String::new());
view!{

  <div style="height:125px;font-family:sans-serif;font-size:14px;padding:25px;border-width:3px;border:solid;">
   <div style="padding:10px 15px">Username:  <input type="text" on:input=move |ev| { set_name(event_target_value(&ev));} prop:value=name /></div>
    
   <div style="padding:10px 15px;">Password:  <input type="password" on:input=move |ev| {set_pw(event_target_value(&ev));} prop:value=pw /></div>
   <div style="position:absolute;padding:10px 20px;left:40%;"> 
    <button on:click= move |_| {

  set_auth(!auth.get());

  //send credentials to server for validation
//set_response_string.set(send_get_request());
  }> Login </button>
</div>
  </div>
}

}




#[component]
fn LoggedIn(auth:ReadSignal<bool>,set_auth:WriteSignal<bool>, response_string:ReadSignal<String>, set_response_string:WriteSignal<String> )-> impl IntoView{



view!{"Logged in"
<button

 on:click= move |_| {

  set_auth(!auth.get());
  }
  >Log Out</button>
  <p>{response_string}</p>
}
}




async fn send_get_request()-> Option<ReadableStream> {
  
    let url = "https://127.0.0.1:3000";

    // Make a GET request
    let resp = Request::get(url).send().await.unwrap();
//resp.body().expect("Ruh roh.").to_string().into()
Some(resp.text().await?)

}
