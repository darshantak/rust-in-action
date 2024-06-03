use yew::prelude::{function_component,html,Html,Properties};
use yew::prelude::*;

#[derive(Properties,PartialEq)]
struct Props{ 
    is_loading: bool
}
#[function_component]
fn App()-> Html{
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter+1;
            counter.set(value);
        }
        };

    html!{
        <div>
            <button {onclick}>{"plus 1"}</button>
            <p>{*counter}</p>
        </div>
    }
}


#[function_component]
fn HelloWorld(props:&Props) -> Html{
    html!{
        <>{"Loading ..."}{props.is_loading.clone()}</>
    }
}

fn main() {
yew::Renderer::<App>::new().render();
}
