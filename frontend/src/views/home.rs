use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div class="box">
            <h2>{"Home"}</h2>
            <p>
                {"Welcome to the ISP Manager! Here you can manage all the data related to your customers and subscriptions."}
            </p>
        </div>
    }
}
