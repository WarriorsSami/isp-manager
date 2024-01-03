use crate::app::Route;
use chrono::{DateTime, Utc};
use common::invoice::InvoiceResponse;
use common::payment::CreatePaymentRequest;
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
    invoices: Option<Vec<InvoiceResponse>>,
    state_invoice_id: u32,
    state_amount: f64,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    CreateRequest,
    CreateResponse(Result<(), anyhow::Error>),
    GetInvoicesRequest,
    GetInvoicesResponse(Result<Vec<InvoiceResponse>, anyhow::Error>),
    EditInvoiceId(u32),
    EditAmount(f64),
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
                            label="Invoice"
                            outlined=true
                            required=true
                            icon="description"
                            onselected={ctx.link().callback(|e: SelectedDetail| {
                                let Single(Some(value)) = e.index else { return Msg::EditInvoiceId(0) };

                                match value {
                                    0 => Msg::EditInvoiceId(0),
                                    value => Msg::EditInvoiceId(value as u32),
                                }
                            })}>
                            {
                                if let Some(invoices) = &self.invoices {
                                    invoices.iter().enumerate().map(|(index, invoice)| {
                                        html! {
                                            <MatListItem value={index.to_string()} graphic={GraphicType::Icon}>{ &invoice.id }</MatListItem>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! {}
                                }
                            }
                    </MatSelect>

                    <MatTextField
                            outlined=true
                            label="Amount"
                            icon="price_change"
                            required=true
                            min="0"
                            field_type={TextFieldType::Number}
                            value={self.state_amount.to_string()}
                            oninput={ctx.link().callback(|value: String| {
                                Msg::EditAmount(value.parse::<f64>().unwrap_or(0.0))
                            })}
                    />
                </div>

                <div class="row-flex">
                    <button class="btn-success" type="submit">
                        <MatButton label="Add" raised=true />
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
        ctx.link().send_message(Msg::GetInvoicesRequest);

        Self {
            invoices: None,
            state_invoice_id: 0,
            state_amount: 0.0,
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::CreateRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = CreatePaymentRequest {
                    invoice_id: self.state_invoice_id,
                    amount: self.state_amount,
                    payment_date: Utc::now().date().and_hms(0, 0, 0),
                };

                let validation_result = state.validate();

                if validation_result.is_err() {
                    link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let payment = state.clone();
                log::info!("Creating payment: {:?}", payment);

                wasm_bindgen_futures::spawn_local(async move {
                    let payment_json = JsValue::from(serde_json::to_string(&payment).unwrap());

                    let create_payment_req = Request::post("http://localhost:8000/api/payment")
                        .header("Content-Type", "application/json")
                        .body(payment_json)
                        .expect("Failed to build request.");

                    let resp = create_payment_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 201 {
                                link.send_message(Msg::CreateResponse(Ok(())));
                            } else {
                                link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                    "Invoice is already paid of payment amount exceeds invoice amount"
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                "Failed to create payment: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::CreateResponse(Ok(_)) => {
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().push(&Route::InvoiceDetail {
                    id: self.state_invoice_id,
                });
                false
            }
            Msg::CreateResponse(Err(err)) => {
                link.send_message(Msg::ToggleLoading);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::GetInvoicesRequest => {
                log::info!("Fetching invoices");

                wasm_bindgen_futures::spawn_local(async move {
                    let get_invoices_req =
                        Request::get("http://localhost:8000/api/invoice?status=UNPAID")
                            .header("Content-Type", "application/json");

                    let resp = get_invoices_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let invoices =
                                    resp.json::<Vec<InvoiceResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {:?}", err)
                                    });

                                link.send_message(Msg::GetInvoicesResponse(invoices));
                            } else {
                                link.send_message(Msg::GetInvoicesResponse(Err(anyhow::anyhow!(
                                    "Failed to get invoices: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetInvoicesResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetInvoicesResponse(Ok(invoices)) => {
                self.invoices = Some(invoices);
                true
            }
            Msg::GetInvoicesResponse(Err(err)) => {
                log::error!("Error: {:?}", err);
                false
            }
            Msg::EditInvoiceId(invoice_id_index) => {
                if let Some(invoices) = self.invoices.as_ref() {
                    let invoice_id = invoices[invoice_id_index as usize].id;
                    self.state_invoice_id = invoice_id;

                    true
                } else {
                    false
                }
            }
            Msg::EditAmount(amount) => {
                self.state_amount = amount;
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
                <h2>{ "Add payment" }</h2>
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
