use leptos::{error::Result, *};




#[component]
pub fn Dashboard(auth:ReadSignal<bool>,set_auth:WriteSignal<bool>, response_string:ReadSignal<String>, set_response_string:WriteSignal<String> )-> impl IntoView{
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





async fn fetch_req() -> Result<String> {
    // make the request
      let res = reqwasm::http::Request::get(&format!(
        "http://127.0.0.1:3000/",
    ))
      .send()
      .await?
      // convert it to text
      .text()
      .await?;
      // return response 
      Ok(res)
  }
  