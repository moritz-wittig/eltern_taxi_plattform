use web_sys::HtmlInputElement;
use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use common::{LoginRequest, LoginRequestWrapper};
use yew_hooks::use_async;

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {

    let login_info = use_state(LoginRequest::default);
    let user_login = {
        let login_info = login_info.clone();
        
        // TODO: Here we now have to forward this user information to the backend
        // probably by simply calling the backend function
        
        // Implementation in the example
        // use_async(async move {
            // let request = LoginRequestWrapper{
            //     user: (*login_info).clone()
            // };
            // login(request).await
        // })

    };

    // in the browser we can only log JavaScript types 
    let oninput_email = {
        // Yew Callback
        Callback::from(move |e: InputEvent| {
            // Extract target of the event + casting it into HtmlInputElement
            let input: HtmlInputElement = e.target_unchecked_into(); 
            log!(JsValue::from(input.value()));
        })
    };

    let oninput_password = {
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            log!(JsValue::from(input.value()))
        })
    };

    html! {
        <div class="auth-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-6 offset-md-3 col-xs-12">
                        <h1 class="text-xs-center">{ "Sign In" }</h1>
                        <p class="text-xs-center">
                            // <Link<AppRoute> to={AppRoute::Register}>
                            //     { "Need an account?" }
                            // </Link<AppRoute>>
                        </p>
                        <form>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="email"
                                        placeholder="Email"
                                        oninput={oninput_email}
                                        />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="password"
                                        placeholder="Password"
                                        oninput={oninput_password}
                                        />
                                </fieldset>
                                <button
                                    class="btn btn-lg btn-primary pull-xs-right"
                                    type="submit"
                                    disabled=false>
                                    { "Sign in" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}