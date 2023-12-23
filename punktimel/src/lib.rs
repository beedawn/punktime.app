use leptos::{error::Result, *};
mod Login;
mod Dashboard;
#[component]
pub fn App() -> impl IntoView {
  let (auth, set_auth) = create_signal(false);
  let (response_string, set_response_string) = create_signal(String::new());
    view!{
        <div>
            <Show
            when=move || auth.get() == false
            fallback=move || view! {  
                <Dashboard::Dashboard auth=auth 
                set_auth=set_auth 
                response_string=response_string 
                set_response_string=set_response_string />
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
                    <Login::Login auth=auth set_auth=set_auth 
                    response_string=response_string 
                    set_response_string=set_response_string  />
                </div>
            </Show>
        </div>   }
}



async fn post_req() -> Result<String> {
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

