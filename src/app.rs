use yew::prelude::*;

#[function_component(App)]

pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class="main-container">
            <h1>{"Counter OwO"}</h1>
            <button {onclick}>{"Click me >w<"}</button>
            <p>{"Pwease cwick me UwU"}</p>
            <p>{"cwicked "}{*counter}{" times OwO so big"}</p>
        </div>
    }
}
