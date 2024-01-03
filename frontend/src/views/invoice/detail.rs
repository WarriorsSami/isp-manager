use crate::app::{AppLink, Route};
use common::invoice::{InvoiceResponse, InvoiceStatus};
use common::payment::PaymentResponse;
use gloo_net::http::Request;
use material_yew::{MatButton, MatCircularProgress};
use yew::{html, AttrValue, Component, Context, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DetailProps {
    pub id: u32,
}

pub struct Detail {
    invoice: Option<InvoiceResponse>,
    payments: Option<Vec<PaymentResponse>>,
}

pub enum Msg {
    GetInvoiceRequest,
    GetInvoiceResponse(Result<InvoiceResponse, anyhow::Error>),
    GetPaymentsRequest,
    GetPaymentsResponse(Result<Vec<PaymentResponse>, anyhow::Error>),
}

impl Detail {
    fn render_invoice(&self, ctx: &Context<Detail>) -> Html {
        if let Some(invoice) = &self.invoice {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Contract ID" }</th>
                            <th>{ "Issue Date" }</th>
                            <th>{ "Due Date" }</th>
                            <th>{ "Amount" }</th>
                            <th>{ "Status" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        <tr>
                            <td>{ &invoice.id }</td>
                            <td>
                                <AppLink to={Route::ContractDetail { id: invoice.contract_id }}>
                                    { &invoice.contract_id }
                                </AppLink>
                            </td>
                            <td>{ invoice.issue_date.format("%Y-%m-%d").to_string() }</td>
                            <td>{ invoice.due_date.format("%Y-%m-%d").to_string() }</td>
                            <td>{ &invoice.amount }</td>
                            <td>{ &invoice.status }</td>
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

    fn render_payments(&self, ctx: &Context<Detail>) -> Html {
        if let Some(payments) = &self.payments {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Amount" }</th>
                            <th>{ "Date" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        { payments.iter().map(|payment| self.render_payment(ctx, payment)).collect::<Html>() }
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

    fn render_payment(&self, ctx: &Context<Detail>, payment: &PaymentResponse) -> Html {
        html! {
            <tr>
                <td>{ &payment.id }</td>
                <td>{ &payment.amount }</td>
                <td>{ payment.payment_date.format("%Y-%m-%d").to_string() }</td>
            </tr>
        }
    }
}

impl Component for Detail {
    type Message = Msg;
    type Properties = DetailProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetInvoiceRequest);
        ctx.link().send_message(Msg::GetPaymentsRequest);

        Self {
            invoice: None,
            payments: None,
        }
    }

    fn update(&mut self, ctx: &yew::Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetInvoiceRequest => {
                log::info!("Fetching invoice with id {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_invoice_req = Request::get(
                        format!("http://localhost:8000/api/invoice/{}", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_invoice_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let invoice = resp.json::<InvoiceResponse>().await.map_err(|err| {
                                    anyhow::anyhow!("Failed to parse response: {}", err)
                                });

                                link.send_message(Msg::GetInvoiceResponse(invoice));
                            } else {
                                link.send_message(Msg::GetInvoiceResponse(Err(anyhow::anyhow!(
                                    "Failed to get invoice: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetInvoiceResponse(Err(anyhow::anyhow!(
                                "Failed to get invoice: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetInvoiceResponse(Ok(invoice)) => {
                self.invoice = Some(invoice);
                true
            }
            Msg::GetInvoiceResponse(Err(err)) => {
                log::error!("Failed to get invoice: {}", err);
                false
            }
            Msg::GetPaymentsRequest => {
                log::info!("Fetching payments for invoice with id {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_payments_req = Request::get(
                        format!("http://localhost:8000/api/invoice/{}/payment", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_payments_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let payments =
                                    resp.json::<Vec<PaymentResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {}", err)
                                    });

                                link.send_message(Msg::GetPaymentsResponse(payments));
                            } else {
                                link.send_message(Msg::GetPaymentsResponse(Err(anyhow::anyhow!(
                                    "Failed to get payments: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetPaymentsResponse(Err(anyhow::anyhow!(
                                "Failed to get payments: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetPaymentsResponse(Ok(payments)) => {
                self.payments = Some(payments);
                true
            }
            Msg::GetPaymentsResponse(Err(err)) => {
                log::error!("Failed to get payments: {}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &yew::Context<Self>) -> Html {
        html! {
            <div class="box">
                <h2>{ "Invoice details" }</h2>
                { self.render_invoice(ctx) }

                <h2>{ "Payments" }</h2>
                {
                    if let Some(InvoiceResponse { status: InvoiceStatus::Unpaid, .. }) = &self.invoice {
                        html! {
                            <h3>
                                <AppLink to={Route::PaymentCreate}>
                                    <MatButton label="Add new payment" icon={AttrValue::from("add")} raised=true />
                                </AppLink>
                            </h3>
                        }
                    } else {
                        html! {}
                    }
                }
                { self.render_payments(ctx) }
            </div>
        }
    }
}
