use gloo_storage::{SessionStorage, Storage};
use web_sys::HtmlInputElement;
use gloo_console::log;
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use common::{LoginRequest, LoginResponse};
use yew_hooks::use_async;
use reqwest::header::CONTENT_TYPE;


/// Login page
#[function_component(Login)]
pub fn login_page() -> Html {
    // UseStateHandle to handle the state of the LoginRequest w/o management of state variables
    let login_info: UseStateHandle<LoginRequest> = use_state(LoginRequest::default);
    let user_login = {
        let login_info = login_info.clone();
        let login_request = (*login_info).clone();
        
        log!(JsValue::from_serde(&login_request).unwrap());
        
        use_async(async move {
            let login_request = (*login_info).clone();
            let client = reqwest::Client::new();

            // Serialize the LoginRequest object into JSON
            let body = serde_json::to_string(&login_request)
                .expect("Failed to serialize LoginRequest to JSON");
            
            log!("body");
            log!(JsValue::from_str(body.as_str()));

            let response = client
            .post("http://localhost:8000/login")
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await;
            
            match response{
                Ok(response) => {
                    match response.status() {
                        reqwest::StatusCode::OK => {
                            // on success, parse our JSON to an APIResponse
                            match response.json::<LoginResponse>().await {
                                Ok(parsed) => {
                                    log!("Success!");
                                    log!(JsValue::from_str(parsed.token.as_str()));
                                    
                                    // Store JWT in local storage
                                    SessionStorage::set("JWT", parsed.token.as_str()).unwrap();

                                },
                                Err(_) => println!("Hm, the response didn't match the shape we expected."),
                            };
                        }
                        reqwest::StatusCode::UNAUTHORIZED => {
                            println!("Need to grab a new token");
                        }
                        other => {
                            panic!("Uh oh! Something unexpected happened: {:?}", other);
                        }
                    };
                    Ok(()) // Return Ok(()) to indicate success
                }
                Err(err) => {
                    // Handle the error
                    log!("Error");
                    log!(JsValue::from_str(err.to_string().as_str()));
                    Err(err.to_string()) // Return the error directly
                }
            }
        })
    };

    let onsubmit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); //prevent form submission from refreshing the page
            user_login.run();                        
        })
    };

    // in the browser we can only log JavaScript types 
    let oninput_email = {
        let login_info = login_info.clone();
        // Yew Callback
        Callback::from(move |e: InputEvent| {
            // Extract target of the event + casting it into HtmlInputElement
            let input: HtmlInputElement = e.target_unchecked_into(); 
            log!(JsValue::from(input.value()));
            
            let mut info = (*login_info).clone();
            info.email = input.value();
            login_info.set(info);
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            log!(JsValue::from(input.value()));

            let mut info = (*login_info).clone();
            info.pw = input.value();
            login_info.set(info);
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
                        <form {onsubmit}>
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