use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::models::conversation::{Conversation, Message};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    let (conversation,set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message{
            user:true,
            text:new_message.clone()
        };
        set_conversation.update(move |c| {
            c.message.push(user_message);
        })
        //TODO converse with the server
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-exp.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>
        // {conversation.get()}
        // <ChatArea />
        <TypeArea send/>
    }
}



