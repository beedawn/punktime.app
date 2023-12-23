use leptos::{error::Result, *};


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
   <div style="padding:10px 15px">Username:  <input type="text" on:input=move |ev| { set_name.set(event_target_value(&ev));} prop:value=name /></div>
    
   <div style="padding:10px 15px;">Password:  <input type="password" on:input=move |ev| {set_pw.set(event_target_value(&ev));} prop:value=pw /></div>
   <div style="position:absolute;padding:10px 20px;left:40%;"> 
    <button on:click= move |_| {

  set_auth.set(!auth.get());

  }> Login </button>
</div>
  </div>
}

}
async fn fetch_req() -> Result<String> {
  // make the request
  let res = reqwasm::http::Request::get(&format!(
      "http://127.0.0.1:3000/",
  ))
  .send()
  .await?
  // convert it to JSON
  .text()
  .await?;
  // extract the URL field for each cat
          Ok(res)

}




#[component]
fn LoggedIn(auth:ReadSignal<bool>,set_auth:WriteSignal<bool>, response_string:ReadSignal<String>, set_response_string:WriteSignal<String> )-> impl IntoView{

  let req = create_local_resource(move || (), |_| fetch_req());
  let fallback = move |errors: RwSignal<Errors>| {
    let error_list = move || {
        errors.with(|errors| {
            errors
                .iter()
                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                .collect_view()
        })
    };

    view! {
        <div class="error">
            <h2>"Error"</h2>
            <ul>{error_list}</ul>
        </div>
    }
};


 let req_view = move || {
        req
    };
view!{
  <div>
  <Transition fallback=move || {
    view! { <div>"Loading (Suspense Fallback)..."</div> }
}>
    <ErrorBoundary fallback>
    <div>
          {req}
    </div>
    </ErrorBoundary>
</Transition>
</div>

<button

 on:click= move |_| {

  set_auth.set(!auth.get());
  }
  >Log Out</button>

}
}




