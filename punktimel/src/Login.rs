use leptos::*;

//returns Login component of a login box with username and password fields, 
//as well as a login button
#[component]
pub fn Login(auth:ReadSignal<bool>, set_auth:WriteSignal<bool>
    ) -> impl IntoView {
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




