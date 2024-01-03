use crate::app::Route;
use common::customer::{CustomerRequest, CustomerResponse};
use gloo_net::http::Request;
use material_yew::text_inputs::TextFieldType;
use material_yew::{MatButton, MatCircularProgress, MatIconButton, MatSnackbar, MatTextField};
use validator::Validate;
use wasm_bindgen::JsValue;
use web_sys::SubmitEvent;
use yew::{html, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Clone, Properties, PartialEq)]
pub struct EditProps {
    pub id: u32,
}

pub struct Edit {
    state_name: String,
    state_fullname: String,
    state_address: String,
    state_phone: String,
    state_cnp: String,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    GetRequest,
    GetResponse(Result<CustomerResponse, anyhow::Error>),
    EditRequest,
    EditResponse(Result<(), anyhow::Error>),
    EditName(String),
    EditFullname(String),
    EditAddress(String),
    EditPhone(String),
    EditCnp(String),
    ShowErrorSnackbar(anyhow::Error),
    HideErrorSnackbar,
    ToggleLoading,
}

impl Edit {
    fn render_form(&self, ctx: &Context<Edit>) -> Html {
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::EditRequest
        });

        html! {
            <form {onsubmit}>
                <div class="form-input">
                    <MatTextField
                        outlined=true
                        label="Name"
                        required=true
                        max_length=20
                        char_counter=true
                        value={self.state_name.clone()}
                        oninput={ctx.link().callback(Msg::EditName)}
                    />

                    <MatTextField
                        outlined=true
                        label="Fullname"
                        required=true
                        max_length=50
                        char_counter=true
                        value={self.state_fullname.clone()}
                        oninput={ctx.link().callback(Msg::EditFullname)}
                    />

                    <MatTextField
                        outlined=true
                        label="Address"
                        required=true
                        max_length=100
                        char_counter=true
                        value={self.state_address.clone()}
                        oninput={ctx.link().callback(Msg::EditAddress)}
                    />

                    <MatTextField
                        outlined=true
                        label="Phone"
                        required=true
                        field_type={TextFieldType::Tel}
                        value={self.state_phone.clone()}
                        oninput={ctx.link().callback(Msg::EditPhone)}
                    />

                    <MatTextField
                        outlined=true
                        label="CNP"
                        required=true
                        pattern={r"^\d{13}$"}
                        value={self.state_cnp.clone()}
                        oninput={ctx.link().callback(Msg::EditCnp)}
                    />
                </div>

                <div class="row-flex">
                    <button class="btn-success" type="submit">
                        <MatButton label="Edit" raised=true />
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

impl Component for Edit {
    type Message = Msg;
    type Properties = EditProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetRequest);

        Self {
            state_name: String::new(),
            state_fullname: String::new(),
            state_address: String::new(),
            state_phone: String::new(),
            state_cnp: String::new(),
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetRequest => {
                wasm_bindgen_futures::spawn_local(async move {
                    let get_customer_req = Request::get(
                        format!("http://localhost:8000/api/customer/{}", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_customer_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let customer = resp.json().await.map_err(|err| {
                                    anyhow::anyhow!("Failed to parse response: {:?}", err)
                                });

                                link.send_message(Msg::GetResponse(customer));
                            } else {
                                link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                    "Failed to get customer: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            log::error!("Failed to send request: {:?}", err);
                            link.send_message(crate::views::customer::edit::Msg::GetResponse(Err(
                                anyhow::anyhow!("Failed to send request: {:?}", err),
                            )));
                        }
                    }
                });
                false
            }
            Msg::GetResponse(Ok(customer)) => {
                log::info!("Customer: {:?}", customer);
                self.state_name = customer.name;
                self.state_fullname = customer.fullname;
                self.state_address = customer.address;
                self.state_phone = customer.phone;
                self.state_cnp = customer.cnp;
                true
            }
            Msg::GetResponse(Err(err)) => {
                log::error!("Failed to get customer: {:?}", err);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::EditRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = CustomerRequest {
                    name: self.state_name.clone(),
                    fullname: self.state_fullname.clone(),
                    address: self.state_address.clone(),
                    phone: self.state_phone.clone(),
                    cnp: self.state_cnp.clone(),
                };

                let validation_result = state.validate();

                if validation_result.is_err() {
                    log::error!("Validation error: {:?}", validation_result);
                    link.send_message(Msg::EditResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let customer = state.clone();
                log::info!("Updating customer: {:?}", customer);

                wasm_bindgen_futures::spawn_local(async move {
                    let customer_json = JsValue::from(serde_json::to_string(&customer).unwrap());

                    let update_customer_req = Request::put(
                        format!("http://localhost:8000/api/customer/{}", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json")
                    .body(customer_json)
                    .expect("Failed to build request.");

                    let resp = update_customer_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                link.send_message(Msg::EditResponse(Ok(())));
                            } else {
                                link.send_message(Msg::EditResponse(Err(anyhow::anyhow!(
                                    "Failed to update customer: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::EditResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });

                false
            }
            Msg::EditResponse(Ok(())) => {
                log::info!("Customer updated");
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().push(&Route::CustomerList);
                false
            }
            Msg::EditResponse(Err(err)) => {
                log::error!("Failed to update customer: {:?}", err);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::EditName(name) => {
                self.state_name = name;
                true
            }
            Msg::EditFullname(fullname) => {
                self.state_fullname = fullname;
                true
            }
            Msg::EditAddress(address) => {
                self.state_address = address;
                true
            }
            Msg::EditPhone(phone) => {
                self.state_phone = phone;
                true
            }
            Msg::EditCnp(cnp) => {
                self.state_cnp = cnp;
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
                <h2>{ "Edit customer" }</h2>
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
