use yew::prelude::*;

#[derive(Debug, Properties, PartialEq)]
pub struct SubscriptionEditProps {
    pub id: u32,
}

#[function_component(Edit)]
pub fn subscription_edit(props: &SubscriptionEditProps) -> Html {
    html! {
        <div>
            <h1>{"Edit subscription "}{props.id}</h1>
        </div>
    }
}
