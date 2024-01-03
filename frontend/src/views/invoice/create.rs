use crate::app::Route;
use chrono::{DateTime, NaiveDate, Utc};
use common::contract::ContractResponse;
use common::invoice::CreateInvoiceRequest;
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
    contracts: Option<Vec<ContractResponse>>,
    state_contract_id: u32,
    state_issue_date: DateTime<Utc>,
    state_due_date: DateTime<Utc>,
    state_amount: f64,
    state_contract_start_date: DateTime<Utc>,
    state_contract_end_date: DateTime<Utc>,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    CreateRequest,
    CreateResponse(Result<(), anyhow::Error>),
    GetContractsRequest,
    GetContractsResponse(Result<Vec<ContractResponse>, anyhow::Error>),
    EditContractId(u32),
    EditIssueDate(DateTime<Utc>),
    EditDueDate(DateTime<Utc>),
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
                     <div class="row-flex">
                         <MatSelect
                            label="Contract"
                            outlined=true
                            required=true
                            icon="description"
                            onselected={ctx.link().callback(|e: SelectedDetail| {
                                let Single(Some(value)) = e.index else { return Msg::EditContractId(0) };

                                match value {
                                    0 => Msg::EditContractId(0),
                                    value => Msg::EditContractId(value as u32),
                                }
                            })}>
                            {
                                if let Some(contracts) = &self.contracts {
                                    contracts.iter().enumerate().map(|(index, contract)| {
                                        html! {
                                            <MatListItem value={index.to_string()} graphic={GraphicType::Icon}>{ &contract.id }</MatListItem>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! {}
                                }
                            }
                         </MatSelect>

                         <MatTextField
                            outlined=true
                            label="Contract start date"
                            icon="event"
                            required=true
                            field_type={TextFieldType::Date}
                            value={self.state_contract_start_date.format("%Y-%m-%d").to_string()}
                         />

                         <MatTextField
                            outlined=true
                            label="Contract end date"
                            icon="event"
                            required=true
                            field_type={TextFieldType::Date}
                            value={self.state_contract_end_date.format("%Y-%m-%d").to_string()}
                         />
                     </div>

                     <MatTextField
                        outlined=true
                        label="Issue date"
                        icon="event"
                        required=true
                        field_type={TextFieldType::Date}
                        value={self.state_issue_date.format("%Y-%m-%d").to_string()}
                        oninput={ctx.link().callback(|value: String| {
                            let date = NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap();
                            Msg::EditIssueDate(DateTime::from_utc(date.and_hms(0, 0, 0), Utc))
                        })}
                    />

                    <MatTextField
                        outlined=true
                        label="Due date"
                        icon="event"
                        required=true
                        field_type={TextFieldType::Date}
                        value={self.state_due_date.format("%Y-%m-%d").to_string()}
                        oninput={ctx.link().callback(|value: String| {
                            let date = NaiveDate::parse_from_str(value.as_str(), "%Y-%m-%d").unwrap();
                            Msg::EditDueDate(DateTime::from_utc(date.and_hms(0, 0, 0), Utc))
                        })}
                    />

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
        ctx.link().send_message(Msg::GetContractsRequest);

        Self {
            contracts: None,
            state_contract_id: 0,
            state_issue_date: Utc::now().date().and_hms(0, 0, 0),
            state_due_date: Utc::now().date().and_hms(0, 0, 0),
            state_amount: 0.0,
            state_contract_start_date: Utc::now().date().and_hms(0, 0, 0),
            state_contract_end_date: Utc::now().date().and_hms(0, 0, 0),
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::CreateRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = CreateInvoiceRequest {
                    contract_id: self.state_contract_id,
                    issue_date: self.state_issue_date,
                    due_date: self.state_due_date,
                    amount: self.state_amount,
                };

                let validation_result = state.validate();

                if validation_result.is_err() {
                    link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let invoice = state.clone();
                log::info!("Creating invoice: {:?}", invoice);

                wasm_bindgen_futures::spawn_local(async move {
                    let invoice_json = JsValue::from(serde_json::to_string(&invoice).unwrap());

                    let create_invoice_req = Request::post("http://localhost:8000/api/invoice")
                        .header("Content-Type", "application/json")
                        .body(invoice_json)
                        .expect("Failed to build request.");

                    let resp = create_invoice_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 201 {
                                link.send_message(Msg::CreateResponse(Ok(())));
                            } else {
                                link.send_message(Msg::CreateResponse(Err(anyhow::anyhow!(
                                    "Invoice period not in contract availability period"
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
                log::info!("Invoice created successfully");
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().push(&Route::ContractDetail {
                    id: self.state_contract_id,
                });
                false
            }
            Msg::CreateResponse(Err(err)) => {
                log::error!("Failed to create invoice: {:?}", err);
                link.send_message(Msg::ToggleLoading);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
            }
            Msg::GetContractsRequest => {
                log::info!("Fetching contracts");

                wasm_bindgen_futures::spawn_local(async move {
                    let get_contracts_req = Request::get("http://localhost:8000/api/contract")
                        .header("Content-Type", "application/json");

                    let resp = get_contracts_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let contracts =
                                    resp.json::<Vec<ContractResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {:?}", err)
                                    });

                                link.send_message(Msg::GetContractsResponse(contracts));
                            } else {
                                link.send_message(Msg::GetContractsResponse(Err(anyhow::anyhow!(
                                    "Failed to get contracts: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetContractsResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetContractsResponse(Ok(contracts)) => {
                self.contracts = Some(contracts);
                true
            }
            Msg::GetContractsResponse(Err(err)) => {
                log::error!("Failed to get contracts: {:?}", err);
                false
            }
            Msg::EditContractId(contract_id_index) => {
                if let Some(contracts) = self.contracts.as_ref() {
                    self.state_contract_start_date =
                        contracts[contract_id_index as usize].start_date;
                    self.state_contract_end_date = contracts[contract_id_index as usize].end_date;

                    let contract_id = contracts[contract_id_index as usize].id;
                    self.state_contract_id = contract_id;

                    log::info!("Selected contract with id: {}", contract_id);
                    log::info!("Contract start date: {}", self.state_contract_start_date);
                    log::info!("Contract end date: {}", self.state_contract_end_date);

                    true
                } else {
                    false
                }
            }
            Msg::EditIssueDate(issue_date) => {
                self.state_issue_date = issue_date;
                true
            }
            Msg::EditDueDate(due_date) => {
                self.state_due_date = due_date;
                true
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
                <h2>{ "Create invoice" }</h2>
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
