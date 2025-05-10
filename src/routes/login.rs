use yew::prelude::*;

#[function_component]
pub fn Login() -> Html {
    html! {
        <div class={
            "w-screen h-screen bg-black text-gray-300 p-4"
        }>
            <h1 class={"text-4xl font-bold"}>{"Sign in"}</h1>
            <label>{"Username:"}</label>
            <input type="text" />
            <label>{"Password"}</label>
            <input type="password"/>
        </div>
    }
}
