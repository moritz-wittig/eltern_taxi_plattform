use web_sys::js_sys::Math::log;
use web_sys::HtmlInputElement;
use gloo_console::log;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {

    // in the browser we can only log JavaScript types 
    let oninput_email = {
        Callback::from(move |e: InputEvent| {
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