use crate::app::Route;
use chrono::{DateTime, NaiveDate, Utc};
use common::contract::CreateContractRequest;
use common::customer::CustomerResponse;
use common::subscription::SubscriptionResponse;
use gloo_net::http::Request;
use material_yew::list::GraphicType;
use material_yew::select::ListIndex::Single;
use material_yew::select::SelectedDetail;
use material_yew::text_inputs::TextFieldType;
use material_yew::{
    MatButton, MatCircularProgress, MatIconButton, MatListItem, MatSelect, MatSnackbar,
    MatTextField,
};
use validator::Validate;
use wasm_bindgen::JsValue;
use web_sys::SubmitEvent;
use yew::{html, Component, Context, Html};
use yew_router::scope_ext::RouterScopeExt;

pub struct Create {
    customers: Option<Vec<CustomerResponse>>,
    subscriptions: Option<Vec<SubscriptionResponse>>,
    state_customer_id: u32,
    state_subscription_id: u32,
    state_start_date: DateTime<Utc>,
    state_end_date: DateTime<Utc>,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    CreateRequest,
    CreateResponse(Result<(), anyhow::Error>),
    GetCustomersRequest,
    GetCustomersResponse(Result<Vec<CustomerResponse>, anyhow::Error>),
    GetSubscriptionsRequest,
    GetSubscriptionsResponse(Result<Vec<SubscriptionResponse>, anyhow::Error>),
    EditCustomerId(u32),
    EditSubscriptionId(u32),
    EditStartDate(DateTime<Utc>),
    EditEndDate(DateTime<Utc>),
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
                    <MatSelect
                        label="Customer"
                        outlined=true
                        required=true
                        icon="person"
                        onselected={ctx.link().callback(|e: SelectedDetail| {
                            let Single(Some(value)) = e.index else { return Msg::EditCustomerId(0) };

                            match value {
                                0 => Msg::EditCustomerId(0),
                                value => Msg::EditCustomerId(value as u32),
                            }
                        })}>
                        {
                            if let Some(customers) = &self.customers {
                                customers.iter().enumerate().map(|(index, customer)| {
                                    html! {
                                        <MatListItem value={index.to_string()} graphic={GraphicType::Icon}>{ &customer.id }</MatListItem>
                                    }
                                }).collect::<Html>()
                            } else {
                                html! {}
                            }
                        }
                    </MatSelect>

                    <MatSelect
                        label="Subscription"
                        outlined=true
                        required=true
                        icon="shop"
                        onselected={ctx.link().callback(|e: SelectedDetail| {
                            let Single(Some(value)) = e.index else { return Msg::EditSubscriptionId(0) };

                            match value {
                                0 => Msg::EditSubscriptionId(0),
                                _ => Msg::EditSubscriptionId(value as u32),
                            }
                        })}>
                        {
                            if let Some(subscriptions) = &self.subscriptions {
                                subscriptions.iter().enumerate().map(|(index, subscription)| {
                                    html! {
                                        <MatListItem value={index.to_string()} graphic={GraphicType::Icon}>{ &subscription.id }</MatListItem>
                                    }
                                }).collect::<Html>()
                            } else {
                                html! {}
                            }
                        }
                    </MatSelect>

                    <MatTextField
                        outlined=true
                        label="Start date"
                        icon="event"
                        required=true
                        field_type={TextFieldType::Date}
                        value={self.state_start_date.format("%Y-%m-%d").to_string()}
                        oninput={ctx.link().callback(|value: String| {
                            let date = NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap();
                            Msg::EditStartDate(DateTime::from_utc(date.and_hms(0, 0, 0), Utc))
                        })}
                    />

                    <MatTextField
                        outlined=true
                        label="End date"
                        icon="event"
                        required=true
                        field_type={TextFieldType::Date}
                        value={self.state_end_date.format("%Y-%m-%d").to_string()}
                        oninput={ctx.link().callback(|value: String| {
                            let date = NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap();
                            Msg::EditEndDate(DateTime::from_utc(date.and_hms(0, 0, 0), Utc))
                        })}
                    />
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

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetCustomersRequest);
        ctx.link().send_message(Msg::GetSubscriptionsRequest);

        Self {
            customers: None,
            subscriptions: None,
            state_customer_id: 0,
            state_subscription_id: 0,
            state_start_date: Utc::now().date().and_hms(0, 0, 0),
            state_end_date: Utc::now().date().and_hms(0, 0, 0),
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::CreateRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = CreateContractRequest {
                    customer_id: self.state_customer_id,
                    subscription_id: self.state_subscription_id,
                    start_date: self.state_start_date,
                    end_date: self.state_end_date,
                };

                log::debug!("State: {:?}", state);

                let validation_result = state.validate();

                if validation_result.is_err() {
                    link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let contract = state.clone();
                log::info!("Creating contract: {:?}", contract);

                wasm_bindgen_futures::spawn_local(async move {
                    let contract_json = JsValue::from(serde_json::to_string(&contract).unwrap());

                    let create_contract_req = Request::post("http://localhost:8000/api/contract")
                        .header("Content-Type", "application/json")
                        .body(contract_json)
                        .expect("Failed to build request.");

                    let resp = create_contract_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 201 {
                                link.send_message(Msg::CreateResponse(Ok(())));
                            } else {
                                link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                    "Failed to create contract: {:?}",
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
                log::info!("Contract created");
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().push(&Route::CustomerDetail {
                    id: self.state_customer_id,
                });
                false
            }
            Msg::CreateResponse(Err(err)) => {
                log::error!("Failed to create contract: {:?}", err);
                link.send_message(Msg::ToggleLoading);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::GetCustomersRequest => {
                wasm_bindgen_futures::spawn_local(async move {
                    let get_customers_req = Request::get("http://localhost:8000/api/customer")
                        .header("Content-Type", "application/json");

                    let resp = get_customers_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let customers = resp.json().await.map_err(|err| {
                                    anyhow::anyhow!("Failed to parse response: {:?}", err)
                                });

                                link.send_message(Msg::GetCustomersResponse(customers));
                            } else {
                                link.send_message(Msg::GetCustomersResponse(Err(anyhow::anyhow!(
                                    "Failed to get customers: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetCustomersResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetCustomersResponse(Ok(customers)) => {
                self.customers = Some(customers);
                true
            }
            Msg::GetCustomersResponse(Err(err)) => {
                log::error!("Failed to retrieve customers: {:?}", err);
                false
            }
            Msg::GetSubscriptionsRequest => {
                wasm_bindgen_futures::spawn_local(async move {
                    let get_subscriptions_req =
                        Request::get("http://localhost:8000/api/subscription")
                            .header("Content-Type", "application/json");

                    let resp = get_subscriptions_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let subscriptions = resp.json().await.map_err(|err| {
                                    anyhow::anyhow!("Failed to parse response: {:?}", err)
                                });

                                link.send_message(Msg::GetSubscriptionsResponse(subscriptions));
                            } else {
                                link.send_message(Msg::GetSubscriptionsResponse(Err(
                                    anyhow::anyhow!("Failed to get subscriptions: {:?}", resp),
                                )));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetSubscriptionsResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetSubscriptionsResponse(Ok(subscriptions)) => {
                self.subscriptions = Some(subscriptions);
                true
            }
            Msg::GetSubscriptionsResponse(Err(err)) => {
                log::error!("Failed to retrieve subscriptions: {:?}", err);
                false
            }
            Msg::EditCustomerId(customer_id_index) => {
                if let Some(customers) = self.customers.as_ref() {
                    let customer_id = customers[customer_id_index as usize].id;
                    self.state_customer_id = customer_id;

                    true
                } else {
                    false
                }
            }
            Msg::EditSubscriptionId(subscription_id) => {
                if let Some(subscriptions) = self.subscriptions.as_ref() {
                    let subscription_id = subscriptions[subscription_id as usize].id;
                    self.state_subscription_id = subscription_id;

                    true
                } else {
                    false
                }
            }
            Msg::EditStartDate(start_date) => {
                log::info!("Start date: {:?}", start_date);
                self.state_start_date = start_date;
                true
            }
            Msg::EditEndDate(end_date) => {
                log::info!("End date: {:?}", end_date);
                self.state_end_date = end_date;
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
