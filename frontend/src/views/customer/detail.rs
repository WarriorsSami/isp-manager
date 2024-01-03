use crate::app::{AppLink, Route};
use common::contract::ContractResponse;
use common::customer::CustomerResponse;
use common::invoice::InvoiceResponse;
use gloo_net::http::Request;
use material_yew::{MatButton, MatCircularProgress, MatIconButton};
use yew::{html, AttrValue, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DetailProps {
    pub id: u32,
}

pub struct Detail {
    customer: Option<CustomerResponse>,
    contracts: Option<Vec<ContractResponse>>,
    unpaid_invoices: Option<Vec<InvoiceResponse>>,
}

pub enum Msg {
    GetRequest,
    GetResponse(Result<CustomerResponse, anyhow::Error>),
    GetContractsRequest,
    GetContractsResponse(Result<Vec<ContractResponse>, anyhow::Error>),
    DeleteContractRequest(u32),
    DeleteContractResponse(Result<(), anyhow::Error>),
    GetUnpaidInvoicesRequest,
    GetUnpaidInvoicesResponse(Result<Vec<InvoiceResponse>, anyhow::Error>),
    DeleteRequest(u32),
    DeleteResponse(Result<(), anyhow::Error>),
}

impl Detail {
    fn render_customer(&self, ctx: &Context<Detail>) -> Html {
        if let Some(customer) = &self.customer {
            let customer_id = customer.id;

            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Name" }</th>
                            <th>{ "Fullname" }</th>
                            <th>{ "Address" }</th>
                            <th>{ "Phone" }</th>
                            <th>{ "CNP" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>

                    <tbody>
                         <tr>
                            <td>{ &customer.id }</td>
                            <td>{ &customer.name }</td>
                            <td>{ &customer.fullname }</td>
                            <td>{ &customer.address }</td>
                            <td>{ &customer.phone }</td>
                            <td>{ &customer.cnp }</td>
                            <td>
                                <AppLink to={Route::CustomerEdit { id: customer.id }}>
                                    <button class="btn-warning">
                                        <MatIconButton icon="edit" />
                                    </button>
                                </AppLink>

                                <button class="btn-danger" onclick={ctx.link().callback(move |_| Msg::DeleteRequest(customer_id))}>
                                    <MatIconButton icon="delete" />
                                </button>
                            </td>
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

    fn render_contracts(&self, ctx: &Context<Detail>) -> Html {
        if let Some(contracts) = &self.contracts {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Subscription ID" }</th>
                            <th>{ "Start date" }</th>
                            <th>{ "End date" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        { for contracts.iter().map(|contract| self.render_contract(ctx, contract)) }
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

    fn render_contract(&self, ctx: &Context<Detail>, contract: &ContractResponse) -> Html {
        let contract_id = contract.id;

        html! {
            <tr>
                <td>{ &contract.id }</td>
                <td>
                    <AppLink to={Route::SubscriptionDetail { id: contract.subscription_id }}>
                        { &contract.subscription_id }
                    </AppLink>
                </td>
                <td>{ contract.start_date.format("%Y-%m-%d").to_string() }</td>
                <td>{ contract.end_date.format("%Y-%m-%d").to_string() }</td>
                <td>
                    <AppLink to={Route::ContractDetail { id: contract.id }}>
                        <button class="btn-info">
                             <MatIconButton icon="info" />
                        </button>
                    </AppLink>

                    <AppLink to={Route::ContractEdit { id: contract.id }}>
                        <button class="btn-warning">
                            <MatIconButton icon="edit" />
                        </button>
                    </AppLink>

                    <button class="btn-danger" onclick={ctx.link().callback(move |_| Msg::DeleteContractRequest(contract_id))}>
                        <MatIconButton icon="delete" />
                    </button>
                </td>
            </tr>
        }
    }

    fn render_unpaid_invoices(&self, ctx: &Context<Detail>) -> Html {
        if let Some(invoices) = &self.unpaid_invoices {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Contract ID" }</th>
                            <th>{ "Issue date" }</th>
                            <th>{ "Due date" }</th>
                            <th>{ "Amount" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        { for invoices.iter().map(|invoice| self.render_invoice(ctx, invoice)) }
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
        html! {
            <tr>
                 <td>{ &invoice.id }</td>
                 <td>{ &invoice.contract_id }</td>
                 <td>{ invoice.issue_date.format("%Y-%m-%d").to_string() }</td>
                 <td>{ invoice.due_date.format("%Y-%m-%d").to_string() }</td>
                 <td>{ &invoice.amount }</td>
                 <td>
                     <AppLink to={Route::InvoiceDetail { id: invoice.id }}>
                         <button class="btn-info">
                             <MatIconButton icon="info" />
                         </button>
                     </AppLink>
                 </td>
            </tr>
        }
    }
}

impl Component for Detail {
    type Message = Msg;
    type Properties = DetailProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetRequest);
        ctx.link().send_message(Msg::GetContractsRequest);
        ctx.link().send_message(Msg::GetUnpaidInvoicesRequest);

        Self {
            customer: None,
            contracts: None,
            unpaid_invoices: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetRequest => {
                log::info!("Requesting customer {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_customer_req = Request::get(
                        format!("http://localhost:8000/api/customer/{}", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_customer_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let customer =
                                    resp.json::<CustomerResponse>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed parsing response: {}", err)
                                    });

                                link.send_message(Msg::GetResponse(customer));
                            } else {
                                link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                    "Failed retrieving customer data: {:?}",
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
            Msg::GetResponse(Ok(customer)) => {
                self.customer = Some(customer);
                true
            }
            Msg::GetResponse(Err(err)) => {
                log::error!("Failed retrieving customer data: {:?}", err);
                self.customer = None;
                true
            }
            Msg::GetContractsRequest => {
                log::info!("Requesting contracts for customer {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_contracts_req = Request::get(
                        format!("http://localhost:8000/api/customer/{}/contract", props.id)
                            .as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_contracts_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let contracts =
                                    resp.json::<Vec<ContractResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed parsing response: {}", err)
                                    });

                                link.send_message(Msg::GetContractsResponse(contracts));
                            } else {
                                link.send_message(Msg::GetContractsResponse(Err(anyhow::anyhow!(
                                    "Failed retrieving contracts data: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetContractsResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
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
                log::error!("Failed retrieving contracts data: {:?}", err);
                self.contracts = None;
                true
            }
            Msg::DeleteContractRequest(id) => {
                log::info!("Deleting contract {}", id);

                wasm_bindgen_futures::spawn_local(async move {
                    let delete_contract_req = Request::delete(
                        format!("http://localhost:8000/api/contract/{}", id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = delete_contract_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 204 {
                                link.send_message(Msg::DeleteContractResponse(Ok(())));
                            } else {
                                link.send_message(Msg::DeleteContractResponse(Err(
                                    anyhow::anyhow!("Failed to delete contract: {:?}", resp),
                                )));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::DeleteContractResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::DeleteContractResponse(Ok(_)) => {
                link.send_message(Msg::GetContractsRequest);
                false
            }
            Msg::DeleteContractResponse(Err(err)) => {
                log::error!("Failed deleting contract: {:?}", err);
                false
            }
            Msg::GetUnpaidInvoicesRequest => {
                log::info!("Requesting unpaid invoices for customer {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_unpaid_invoices_req = Request::get(
                        format!("http://localhost:8000/api/customer/{}/invoice", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_unpaid_invoices_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let invoices =
                                    resp.json::<Vec<InvoiceResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed parsing response: {}", err)
                                    });

                                link.send_message(Msg::GetUnpaidInvoicesResponse(invoices));
                            } else {
                                link.send_message(Msg::GetUnpaidInvoicesResponse(Err(
                                    anyhow::anyhow!(
                                        "Failed retrieving unpaid invoices data: {:?}",
                                        resp
                                    ),
                                )));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetUnpaidInvoicesResponse(Err(
                                anyhow::anyhow!("Failed to send request: {}", err),
                            )));
                        }
                    }
                });

                false
            }
            Msg::GetUnpaidInvoicesResponse(Ok(invoices)) => {
                self.unpaid_invoices = Some(invoices);
                true
            }
            Msg::GetUnpaidInvoicesResponse(Err(err)) => {
                log::error!("Failed retrieving unpaid invoices data: {:?}", err);
                self.unpaid_invoices = None;
                true
            }
            Msg::DeleteRequest(id) => {
                log::info!("Deleting customer with id {}", id);

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let delete_customer_req = Request::delete(
                        format!("http://localhost:8000/api/customer/{}", id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = delete_customer_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 204 {
                                link.send_message(Msg::DeleteResponse(Ok(())));
                            } else {
                                link.send_message(Msg::DeleteResponse(Err(anyhow::anyhow!(
                                    "Failed to delete customer: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::DeleteResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::DeleteResponse(Ok(_)) => {
                link.navigator().unwrap().push(&Route::CustomerList);
                false
            }
            Msg::DeleteResponse(Err(err)) => {
                log::error!("Failed to delete customer: {:?}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <h2>{ "Customer details" }</h2>
                { self.render_customer(ctx) }

                <h2>{ "Contracts" }</h2>
                <h3>
                    <AppLink to={Route::ContractCreate}>
                        <MatButton label="Create new contract" icon={AttrValue::from("add")} raised=true />
                    </AppLink>
                </h3>
                { self.render_contracts(ctx) }

                <h2>{ "Unpaid invoices" }</h2>
                { self.render_unpaid_invoices(ctx) }
            </div>
        }
    }
}
