use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class={
            "w-screen h-screen bg-black text-gray-300 p-4"
        }>
            <h1 class={"text-4xl font-bold"}>{"Not Found"}</h1>
            <p>{"404"}</p>
            <p>{"Server status: OK"}</p>
        </div>
    }
}
