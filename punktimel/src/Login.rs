use leptos::*;
use leptos::error::Error;

//returns Login component of a login box with username and password fields, 
//as well as a login button
#[component]
pub fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>
    ) -> impl IntoView {
    let (username, set_username) = create_signal(String::new());
    let (pw, set_pw) = create_signal(String::new());

    
view!{
    //Login box
    <div style="height:125px;font-family:sans-serif;font-size:14px;padding:25px;border-width:3px;border:solid;">
        //user input 
        <div style="padding:10px 15px">
            Username:  
            <input type="text" on:input=move |ev| { 
            set_username.set(event_target_value(&ev));
            } prop:value=username />
        </div>
        //password input
        <div style="padding:10px 15px;">
            Password:  
            <input type="password" on:input=move |ev| {
                set_pw.set(event_target_value(&ev));
            } prop:value=pw />
        </div>
        <div style="position:absolute;padding:10px 20px;left:40%;"> 
            //confirm button
            <button on:click= move |_| {
            //need to add authenication feature this just toggles the auth boolean
            

let result = post_req();


            set_auth.set(!auth.get());
            }> Login </button>
        </div>
    </div>
    }
}


async fn post_req() -> Result<String, Error> {
    // make the request
      let res = reqwasm::http::Request::post(&format!(
        "http://127.0.0.1:3000/login",
    ))
      .send()
      .await?
      // convert it to JSON
      .text()
      .await?;
      // return response 
      Ok(res)
  }



