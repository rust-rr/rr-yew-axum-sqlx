use self::components::header::Header;
use yew::prelude::*;

mod components;
mod pages;
mod router;
mod store;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
            <Header />
            <div class="flex flex-col justify-center items-center">
                <h1 class="my-4 text-4xl font-bold">{"Hello World"}</h1>
                <button
                    class="px-4 py-2 rounded-md text-white bg-indigo-500 hover:bg-indigo-600 focus:ring"
                    {onclick}
                >
                    {"Click to +1"}
                </button>
                <p class="my-4 text-2xl font-semibold text-purple-500">{*counter}</p>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
