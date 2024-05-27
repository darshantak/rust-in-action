use yew::prelude::*;
// use wasm_bindgen::prelude::*;
use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use reqwasm::http::Request;

#[derive(Deserialize,Debug,Clone)]
struct Message{
    content:String,
}

#[function_component(App)]
fn app() -> Html{
    let message = use_state(|| None);
    {
        let message = message.clone();
use_effect( move || {
    spawn_local(async move{
        let fetched_message : Message= Request::get("http://127.0.0.1:8080/api/message")
        .send()
        .await
        .unwrap()
        .json()
        .await.unwrap();
    message.set(Some(fetched_message))
    });
    || ()
});
// println!("{:?}",&message);
    }
    html!(
        <div>
            <h1> {"Yew Frontend"} </h1>
            {
                if let Some(msg) = &*message{
                    html! {<h1> { &msg.content }</h1>}
                    // println!(&msg.content)
                }else{
                    html! {<h1>{"Loading..."}</h1>}
                }
            }
        </div>
    )

}