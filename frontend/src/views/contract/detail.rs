use crate::app::{AppLink, Route};
use common::contract::ContractResponse;
use common::invoice::InvoiceResponse;
use gloo_net::http::Request;
use material_yew::{MatButton, MatCircularProgress, MatIconButton};
use yew::{html, AttrValue, Component, Context, Html, Properties};

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DetailProps {
    pub id: u32,
}

pub struct Detail {
    contract: Option<ContractResponse>,
    invoices: Option<Vec<InvoiceResponse>>,
}

pub enum Msg {
    GetContractRequest,
    GetContractResponse(Result<ContractResponse, anyhow::Error>),
    GetInvoicesRequest,
    GetInvoicesResponse(Result<Vec<InvoiceResponse>, anyhow::Error>),
    DeleteInvoiceRequest(u32),
    DeleteInvoiceResponse(Result<(), anyhow::Error>),
}

impl Detail {
    fn render_contract(&self, ctx: &Context<Detail>) -> Html {
        if let Some(contract) = &self.contract {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Customer ID" }</th>
                            <th>{ "Subscription ID" }</th>
                            <th>{ "Start Date" }</th>
                            <th>{ "End Date" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr>
                            <td>{ &contract.id }</td>
                            <td>
                                <AppLink to={Route::CustomerDetail { id: contract.customer_id }}>
                                    { &contract.customer_id }
                                </AppLink>
                            </td>
                            <td>
                                <AppLink to={Route::SubscriptionDetail { id: contract.subscription_id }}>
                                    { &contract.subscription_id }
                                </AppLink>
                            </td>
                            <td>{ contract.start_date.format("%Y-%m-%d").to_string() }</td>
                            <td>{ &contract.end_date.format("%Y-%m-%d").to_string() }</td>
                        </tr>
                    </tbody>
                </table>
            }
        } else {
            html! {
                <div>
                    <MatCircularProgress indeterminate=true />
                </div>
            }
        }
    }

    fn render_invoices(&self, ctx: &Context<Detail>) -> Html {
        if let Some(invoices) = &self.invoices {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Issue Date" }</th>
                            <th>{ "Due Date" }</th>
                            <th>{ "Amount" }</th>
                            <th>{ "Status" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        { invoices.iter().map(|invoice| self.render_invoice(ctx, invoice)).collect::<Html>() }
                    </tbody>
                </table>
            }
        } else {
            html! {
                <div>
                    <MatCircularProgress indeterminate=true />
                </div>
            }
        }
    }

    fn render_invoice(&self, ctx: &Context<Detail>, invoice: &InvoiceResponse) -> Html {
        let invoice_id = invoice.id;

        html! {
            <tr>
                 <td>{ &invoice.id }</td>
                 <td>{ invoice.issue_date.format("%Y-%m-%d").to_string() }</td>
                 <td>{ invoice.due_date.format("%Y-%m-%d").to_string() }</td>
                 <td>{ &invoice.amount }</td>
                 <td>{ &invoice.status }</td>
                 <td>
                     <AppLink to={Route::InvoiceDetail { id: invoice.id }}>
                         <button class="btn-info">
                             <MatIconButton icon="info" />
                         </button>
                     </AppLink>

                     <button class="btn-danger" onclick={ctx.link().callback(move |_| Msg::DeleteInvoiceRequest(invoice_id))}>
                         <MatIconButton icon="delete" />
                     </button>
                 </td>
            </tr>
        }
    }
}

impl Component for Detail {
    type Message = Msg;
    type Properties = DetailProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetContractRequest);
        ctx.link().send_message(Msg::GetInvoicesRequest);

        Self {
            contract: None,
            invoices: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetContractRequest => {
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

                                link.send_message(Msg::GetContractResponse(contract));
                            } else {
                                link.send_message(Msg::GetContractResponse(Err(anyhow::anyhow!(
                                    "Failed to get contract: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetContractResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetContractResponse(Ok(contract)) => {
                self.contract = Some(contract);
                true
            }
            Msg::GetContractResponse(Err(err)) => {
                log::error!("Failed retrieving contract data: {:?}", err);
                false
            }
            Msg::GetInvoicesRequest => {
                log::info!("Fetching invoices for contract with id {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_invoices_req = Request::get(
                        format!("http://localhost:8000/api/contract/{}/invoice", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_invoices_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let invoices =
                                    resp.json::<Vec<InvoiceResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {}", err)
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
                                "Failed to send request: {}",
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
                log::error!("Failed retrieving invoices data: {:?}", err);
                false
            }
            Msg::DeleteInvoiceRequest(id) => {
                log::info!("Deleting invoice with id {}", id);

                wasm_bindgen_futures::spawn_local(async move {
                    let delete_invoice_req = Request::delete(
                        format!("http://localhost:8000/api/invoice/{}", id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = delete_invoice_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 204 {
                                link.send_message(Msg::DeleteInvoiceResponse(Ok(())));
                            } else {
                                link.send_message(Msg::DeleteInvoiceResponse(Err(
                                    anyhow::anyhow!("Failed to delete invoice: {:?}", resp),
                                )));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::DeleteInvoiceResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::DeleteInvoiceResponse(Ok(_)) => {
                link.send_message(Msg::GetInvoicesRequest);
                false
            }
            Msg::DeleteInvoiceResponse(Err(err)) => {
                log::error!("Failed to delete invoice: {:?}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <h2>{ "Contract details" }</h2>
                { self.render_contract(ctx) }

                <h2>{ "Invoices" }</h2>
                <h3>
                    <AppLink to={Route::InvoiceCreate}>
                        <MatButton label="Create new invoice" icon={AttrValue::from("add")} raised=true />
                    </AppLink>
                </h3>
                { self.render_invoices(ctx) }
            </div>
        }
    }
}
