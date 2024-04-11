#[component]
pub fn TypeArea(cx: Scope, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<DIV>(cx);
    view! {
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 bg-white border-t border-g">
            <form class="w-full flex justify-center items-center gap-4" on:submit=move |ev| {
            ev.prevent_default();
            let input = input_ref.get().expect("input to exist");
            send.dispatch(input.value());
            input.set_value("");
        }>
            <input type="text" class="w-2/3 p-4 border-gray-300 rounded-full" node_ref=input_ref/>
            <input type="submit" class="h-full p-4 bg-blue-50 text-white rounded-full cursor-pointer"/>
            </form>
        </div>
    }
}