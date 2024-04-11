use leptos::*;
const USER_MESSAGE_CLASS: &str= "max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 rounded-lg self-start bg-gray-200 text-gray-700";
#[component]
pub fn ChatArea(cx: Scope, conversation: ReadSignal<Conversation>) -> impl IntoView {
    let input_ref = create_node_ref::<DIV>(cx);

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    })

    view! {cx,
        <div>
            {
                move || conversation.get().messages.iter().map(move |message| {
                    view! {cx,
                        <div class={class_str}>
                            {message.text.clone()}
                        </div>
                    }
                }).collect::<Vec<_>>()
                }
            }
        </div>
    }
}