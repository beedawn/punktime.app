use leptos::{error::Result, *};

#[component]
pub fn App() -> impl IntoView {
  let (auth, set_auth) = create_signal(false);
  let (response_string, set_response_string) = create_signal(String::new());
    view!{
        <div>
            <Show
            when=move || auth.get() == false
            fallback=move || view! {  
                <Dashboard auth=auth 
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
                    <Login auth=auth set_auth=set_auth 
                    response_string=response_string 
                    set_response_string=set_response_string  />
                </div>
            </Show>
        </div>   }
}

//returns Login component of a login box with username and password fields, 
//as well as a login button
#[component]
fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>, 
    response_string:ReadSignal<String>, 
    set_response_string:WriteSignal<String>) -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (pw, set_pw) = create_signal(String::new());
view!{
    //Login box
    <div style="height:125px;font-family:sans-serif;font-size:14px;padding:25px;border-width:3px;border:solid;">
        //user input 
        <div style="padding:10px 15px">
            Username:  
            <input type="text" on:input=move |ev| { 
            set_name.set(event_target_value(&ev));
            } prop:value=name />
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
    // return response 
    Ok(res)
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


#[component]
fn Dashboard(auth:ReadSignal<bool>,set_auth:WriteSignal<bool>, response_string:ReadSignal<String>, set_response_string:WriteSignal<String> )-> impl IntoView{
view!{
    <div>
        <APIRequest />
    </div>
    <button
    on:click= move |_| {
        //flips auth state
        set_auth.set(!auth.get());
        }
    >Log Out</button>
    }
}

#[component]
fn APIRequest()-> impl IntoView {
    let req = create_local_resource(move || (), |_| post_req());
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
              <ul>{error_list}</ul>
          </div>
        }
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
    }
}