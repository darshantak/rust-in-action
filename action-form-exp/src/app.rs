use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main id="app">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[server]
async fn something(is_error: Option<String>) -> Result<String, ServerFnError>{
    if is_error.is_none(){
        Ok(String::from("Successfull Submit"))
    } else{
        Err(ServerFnError::ServerError(String::from("Server error")))
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    //creating a signal so that client can trigger this
    let something_action = Action::<Something, _>::server();
    //deriving this signal will get us the value of the signal
    let value_from_action = Signal::derive(move || {
        something_action.value().get().unwrap_or_else(|| Ok(String::from("This is from the variable value_from_action")))
    });
    //now for logging at the server level we can use side effects which we can define by new_isomorphic
    Effect::new_isomorphic(move |_|  logging::log!("Got value : {:?}",value_from_action.get()));

    view! {
        <h1>"Testing the action form"</h1>
        <ErrorBoundary fallback=move |error| format!("{:?}",error.get().into_iter().next().unwrap().1.to_string())>
        {value_from_action}
        <ActionForm action=something_action class="form">
        <label>"Is Error"<input type="checkbox" name="is_error" /></label>
        <button type="submit">"Submit"</button>
        </ActionForm>
        </ErrorBoundary>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
