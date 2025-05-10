use yew::prelude::*;

#[function_component]
pub fn Echo() -> Html {
    #[allow(clippy::redundant_closure)]
    let initial_state = || String::new();
    let username = use_state(initial_state);
    let password = use_state(initial_state);
    let response = use_state(initial_state);

    html! {
        <div class="p-4 bg-zinc-900 text-gray-400 w-screen h-screen flex items-center justify-center gap-2 flex-col">
            <h1 class={"text-white text-4xl"}>{"Login"}</h1>
            <form class="flex items-center justify-center gap-2 flex-col">
                <label for="username">{"Username"}</label>
                <input type="text" id="username" name="username" class={"py-1 px-2 bg-gray-500 text-gray-50 border-zinc-500 border-2 rounded-lg"} />
                <label for="password">{"Password"}</label>
                <input type="password" id="password" name="password" class={"py-1 px-2 bg-gray-500 text-gray-50 border-zinc-500 border-2 rounded-lg"} />
                <button type="submit" class={"bg-cyan-500 p-1 rounded-lg text-gray-200 font-bold"} >{"Submit"}</button>
            </form>
        </div>
    }
}
