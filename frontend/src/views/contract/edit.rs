use chrono::{DateTime, NaiveDate, Utc};
use gloo_net::http::Request;
use material_yew::text_inputs::TextFieldType;
use material_yew::{
    MatButton, MatCircularProgress, MatIconButton, MatSnackbar,
    MatTextField,
};
use validator::Validate;
use wasm_bindgen::JsValue;
use web_sys::SubmitEvent;
use yew::{html, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;
use common::contract::{ContractResponse, UpdateContractRequest};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct EditProps {
    pub id: u32,
}

pub struct Edit {
    state_start_date: DateTime<Utc>,
    state_end_date: DateTime<Utc>,
    state_error: Option<String>,
    state_loading: bool,
}

pub enum Msg {
    GetRequest,
    GetResponse(Result<ContractResponse, anyhow::Error>),
    EditRequest,
    EditResponse(Result<(), anyhow::Error>),
    EditStartDate(DateTime<Utc>),
    EditEndDate(DateTime<Utc>),
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
            state_start_date: Utc::now().date().and_hms(0, 0, 0),
            state_end_date: Utc::now().date().and_hms(0, 0, 0),
            state_error: None,
            state_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetRequest => {
                log::info!("Fetching contract with id {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_contract_req = Request::get(
                        format!("http://localhost:8000/api/contract/{}", props.id).as_str(),
                    )
                        .header("Content-Type", "application/json");

                    let resp = get_contract_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let contract =
                                    resp.json::<ContractResponse>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {}", err)
                                    });

                                link.send_message(Msg::GetResponse(contract));
                            } else {
                                link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                    "Failed to get contract: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetResponse(Ok(contract)) => {
                self.state_start_date = contract.start_date;
                self.state_end_date = contract.end_date;
                true
            }
            Msg::GetResponse(Err(err)) => {
                log::error!("Failed retrieving contract data: {:?}", err);
                false
            }
            Msg::EditRequest => {
                link.send_message(Msg::ToggleLoading);

                let state = UpdateContractRequest {
                    start_date: self.state_start_date,
                    end_date: self.state_end_date,
                };

                log::debug!("State: {:?}", state);

                let validation_result = state.validate();

                if validation_result.is_err() {
                    link.send_message(Msg::EditResponse(Err(anyhow::anyhow!(
                        "Validation failed: {:?}",
                        validation_result
                    ))));
                    return false;
                }

                let contract = state.clone();
                log::info!("Creating contract: {:?}", contract);

                wasm_bindgen_futures::spawn_local(async move {
                    let contract_json = JsValue::from(serde_json::to_string(&contract).unwrap());

                    let create_contract_req = Request::put(format!("http://localhost:8000/api/contract/{}", props.id).as_str())
                        .header("Content-Type", "application/json")
                        .body(contract_json)
                        .expect("Failed to build request.");

                    let resp = create_contract_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                link.send_message(Msg::EditResponse(Ok(())));
                            } else {
                                link.send_message(Msg::EditResponse(Err(anyhow::anyhow!(
                                    "Failed to create contract: {:?}",
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
                log::info!("Contract created");
                link.send_message(Msg::ToggleLoading);
                link.navigator().unwrap().back();
                false
            }
            Msg::EditResponse(Err(err)) => {
                log::error!("Failed to create contract: {:?}", err);
                link.send_message(Msg::ToggleLoading);
                link.send_message(Msg::ShowErrorSnackbar(err));
                false
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
                <h2>{ "Edit subscription" }</h2>
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

