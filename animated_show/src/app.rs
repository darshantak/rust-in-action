use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use core::time::Duration;
#[component]
pub fn App() -> impl IntoView {
    
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/animated_show.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                <Route path="" view=AnimatedShowComp/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}
#[component]
fn AnimatedShowComp()-> impl IntoView{
    let show = create_rw_signal(false);
    view! {
        <div 
        class="hover-me" 
        on:mouseenter=move |_| show.set(true)
        on:mouseleave=move |_| show.set(false)
        > 
        "Hover Me"
        </div>
        <AnimatedShow 
        when=show
        show_class="fade-in-1000"
        hide_class="fade-out-1000"
        hide_delay=Duration::from_millis(10) >
            <div class="here-i-am">
                "Here I am !"
            </div>
        </AnimatedShow>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {

    // #[cfg(feature = "ssr")]
    // {
    //     // this can be done inline because it's synchronous
    //     // if it were async, we'd use a server function
    //     let resp = expect_context::<leptos_actix::ResponseOptions>();
    //     resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    // }
    
    view! {
        <h1>"Not Found"</h1>
    }
}
