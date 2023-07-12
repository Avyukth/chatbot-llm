use leptos::{*, html::Div};
use crate::model::conversation::Conversation;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn ChatArea(cx:Scope, conversation:ReadSignal<Conversation>) -> impl IntoView {
    let messages = conversation.map(|c| c.messages.clone());
    view! {cx,
        <div class="flex flex-col h-full">
            <div class="flex-grow overflow-y-auto">
                <div class="flex flex-col justify-end h-full">
                    {for messages.into_iter().map(|m| {
                        if m.user {
                            view! {<div class=USER_MESSAGE_CLASS>{m.text}</div>}
                        } else {
                            view! {<div class=MODEL_MESSAGE_CLASS>{m.text}</div>}
                        }
                    })}
                </div>
            </div>
        </div>
    }
}
