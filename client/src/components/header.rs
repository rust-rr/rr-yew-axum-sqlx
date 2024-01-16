use yew::prelude::*;

#[function_component(Header)]
pub fn header_component() -> Html {
    html! {
        <div class="flex justify-between items-center bg-slate-700 text-white px-3 py-2">
            <div>{"Left Label"}</div>
            <div>{"Right Label"}</div>
        </div>
    }
}
