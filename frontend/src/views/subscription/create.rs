use crate::app::Route;
use common::subscription::{SubscriptionRequest, SubscriptionType};
use gloo_net::http::Request;
use material_yew::list::{GraphicType, SelectedDetail};
use material_yew::select::ListIndex::Single;
use material_yew::text_inputs::{TextAreaCharCounter, TextFieldType};
use material_yew::{
    MatButton, MatCircularProgress, MatIconButton, MatListItem, MatSelect, MatSnackbar,
    MatTextArea, MatTextField,
};
use validator::Validate;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_router::prelude::RouterScopeExt;

pub struct Create {
    state_description: String,
    state_subscription_type: SubscriptionType,
    state_traffic: i32,
    state_price: f64,
    state_extra_traffic_price: f64,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    CreateRequest,
    CreateResponse(Result<(), anyhow::Error>),
    EditDescription(String),
    EditSubscriptionType(SubscriptionType),
    EditTraffic(i32),
    EditPrice(f64),
    EditExtraTrafficPrice(f64),
    ShowErrorSnackbar(anyhow::Error),
    HideErrorSnackbar,
    ToggleLoading,
}

impl Create {
    fn render_form(&self, ctx: &Context<Create>) -> Html {
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::CreateRequest
        });

        html! {
            <form {onsubmit}>
                <div class="form-input">
                    <MatTextArea
                        outlined=true
                        label="Description"
                        required=true
                        value={self.state_description.clone()}
                        max_length=100
                        char_counter={TextAreaCharCounter::Internal}
                        oninput={ctx.link().callback(Msg::EditDescription)} />

                    <MatSelect
                        label="Type"
                        outlined=true
                        required=true
                        icon="shop"
                        onselected={ctx.link().callback(|e: SelectedDetail| {
                            let Single(Some(value)) = e.index else { return Msg::EditSubscriptionType(SubscriptionType::Mobile); };

                            match value {
                                1 => Msg::EditSubscriptionType(SubscriptionType::Mobile),
                                2 => Msg::EditSubscriptionType(SubscriptionType::Fixed),
                                3 => Msg::EditSubscriptionType(SubscriptionType::Tv),
                                4 => Msg::EditSubscriptionType(SubscriptionType::MobileInternet),
                                5 => Msg::EditSubscriptionType(SubscriptionType::FixedInternet),
                                _ => Msg::EditSubscriptionType(SubscriptionType::Mobile),
                            }
                        })}>
                        <MatListItem>{""}</MatListItem>
                        <MatListItem value="1" graphic={GraphicType::Icon}>{SubscriptionType::Mobile}</MatListItem>
                        <MatListItem value="2" graphic={GraphicType::Icon}>{SubscriptionType::Fixed}</MatListItem>
                        <MatListItem value="3" graphic={GraphicType::Icon}>{SubscriptionType::Tv}</MatListItem>
                        <MatListItem value="4" graphic={GraphicType::Icon}>{SubscriptionType::MobileInternet}</MatListItem>
                        <MatListItem value="5" graphic={GraphicType::Icon}>{SubscriptionType::FixedInternet}</MatListItem>
                    </MatSelect>

                    <MatTextField
                        label="Traffic"
                        value={self.state_traffic.to_string()}
                        icon="wifi"
                        field_type={TextFieldType::Number}
                        min="0"
                        outlined=true
                        required=true
                        oninput={ctx.link().callback(|value: String| {
                            let value = value.parse::<i32>().unwrap_or(0);
                            Msg::EditTraffic(value)
                        })} />

                    <MatTextField
                        label="Price"
                        value={self.state_price.to_string()}
                        icon="price_change"
                        field_type={TextFieldType::Number}
                        min="0"
                        outlined=true
                        required=true
                        oninput={ctx.link().callback(|value: String| {
                            let value = value.parse::<f64>().unwrap_or(0.0);
                            Msg::EditPrice(value)
                        })} />

                    <MatTextField
                        label="Extra Traffic Price"
                        value={self.state_extra_traffic_price.to_string()}
                        icon="price_change"
                        field_type={TextFieldType::Number}
                        min="0"
                        outlined=true
                        required=true
                        oninput={ctx.link().callback(|value: String| {
                            let value = value.parse::<f64>().unwrap_or(0.0);
                            Msg::EditExtraTrafficPrice(value)
                        })} />
                </div>

                <div class="loading-box">
                    <button class="btn-success" type="submit">
                        <MatButton label="Create" raised=true />
                    </button>

                    {
                        if self.state_loading {
                            html! {
                                <MatCircularProgress indeterminate=true />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </form>
        }
    }
}

impl Component for Create {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state_description: String::new(),
            state_subscription_type: SubscriptionType::Mobile,
            state_traffic: 0,
            state_price: 0.0,
            state_extra_traffic_price: 0.0,
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::CreateRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = SubscriptionRequest {
                    description: self.state_description.clone(),
                    subscription_type: self.state_subscription_type.clone(),
                    traffic: self.state_traffic,
                    price: self.state_price,
                    extra_traffic_price: self.state_extra_traffic_price,
                };

                let validation_result = state.validate();

                if validation_result.is_err() {
                    link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let subscription = state.clone();
                log::info!("Creating subscription: {:?}", subscription);

                wasm_bindgen_futures::spawn_local(async move {
                    let subscription_json =
                        JsValue::from(serde_json::to_string(&subscription).unwrap());

                    let create_subscription_req =
                        Request::post("http://localhost:8000/api/subscription")
                            .header("Content-Type", "application/json")
                            .body(subscription_json)
                            .expect("Failed to build request.");

                    let resp = create_subscription_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 201 {
                                link.send_message(Msg::CreateResponse(Ok(())));
                            } else {
                                link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                    "Failed to create subscription: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::CreateResponse(Ok(())) => {
                log::info!("Subscription created");
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().push(&Route::SubscriptionList);
                false
            }
            Msg::CreateResponse(Err(err)) => {
                log::error!("Failed to create subscription: {:?}", err);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::EditDescription(description) => {
                self.state_description = description;
                true
            }
            Msg::EditSubscriptionType(subscription_type) => {
                self.state_subscription_type = subscription_type;
                true
            }
            Msg::EditTraffic(traffic) => {
                self.state_traffic = traffic;
                true
            }
            Msg::EditPrice(price) => {
                self.state_price = price;
                true
            }
            Msg::EditExtraTrafficPrice(extra_traffic_price) => {
                self.state_extra_traffic_price = extra_traffic_price;
                true
            }
            Msg::ShowErrorSnackbar(err) => {
                self.state_error = Some(err.to_string());
                true
            }
            Msg::HideErrorSnackbar => {
                self.state_error = None;
                true
            }
            Msg::ToggleLoading => {
                self.state_loading = !self.state_loading;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();

        html! {
            <div class="box">
                <h2>{ "Create subscription" }</h2>
                { self.render_form(ctx) }

                <MatSnackbar
                    open={self.state_error.is_some()}
                    label_text={self.state_error.clone().unwrap_or("".to_string())}
                    stacked=true>

                    <span onclick={link.callback(|_| Msg::HideErrorSnackbar)} class="snackbar-dismiss-slot" slot="dismiss">
                        <MatIconButton icon="close" />
                    </span>
                </MatSnackbar>
            </div>
        }
    }
}
