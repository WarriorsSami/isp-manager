use crate::app::{AppLink, Route};
use common::customer::CustomerResponse;
use gloo_net::http::Request;
use material_yew::{MatButton, MatCircularProgress, MatIconButton};
use yew::{html, AttrValue, Component, Context, Html};

pub struct List {
    customers: Option<Vec<CustomerResponse>>,
}

pub enum Msg {
    GetAllRequest,
    GetAllResponse(Result<Vec<CustomerResponse>, anyhow::Error>),
    DeleteRequest(u32),
    DeleteResponse(Result<(), anyhow::Error>),
}

impl List {
    fn render_table(&self, ctx: &Context<List>) -> Html {
        if let Some(customers) = &self.customers {
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
                        { customers.iter().map(|customer| self.render_item(ctx, customer)).collect::<Html>() }
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

    fn render_item(&self, ctx: &Context<List>, customer: &CustomerResponse) -> Html {
        let customer_id = customer.id;

        html! {
            <tr>
                <td>{ &customer.id }</td>
                <td>{ &customer.name }</td>
                <td>{ &customer.fullname }</td>
                <td>{ &customer.address }</td>
                <td>{ &customer.phone }</td>
                <td>{ &customer.cnp }</td>
                <td>
                    <AppLink to={Route::CustomerDetail { id: customer.id }}>
                        <button class="btn-info">
                            <MatIconButton icon="info" />
                        </button>
                    </AppLink>

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
        }
    }
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetAllRequest);

        Self { customers: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::GetAllRequest => {
                log::info!("Requesting all customers");

                wasm_bindgen_futures::spawn_local(async move {
                    let get_customers_req = Request::get("http://localhost:8000/api/customer")
                        .header("Content-Type", "application/json");

                    let resp = get_customers_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let customers =
                                    resp.json::<Vec<CustomerResponse>>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {}", err)
                                    });

                                link.send_message(Msg::GetAllResponse(customers));
                            } else {
                                link.send_message(Msg::GetAllResponse(Err(anyhow::anyhow!(
                                    "Failed to get customers: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetAllResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetAllResponse(Ok(customers)) => {
                self.customers = Some(customers);
                true
            }
            Msg::GetAllResponse(Err(_)) => false,
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
                link.send_message(Msg::GetAllRequest);
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
                <h2>{ "Customers" }</h2>
                <h3>
                    <AppLink to={Route::CustomerCreate}>
                        <MatButton label="Create new customer" icon={AttrValue::from("add")} raised=true />
                    </AppLink>
                </h3>
                { self.render_table(ctx) }
            </div>
        }
    }
}
