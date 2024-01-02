use crate::app::{AppLink, Route};
use common::subscription::SubscriptionResponse;
use gloo_net::http::Request;
use material_yew::{MatButton, MatCircularProgress, MatIconButton};
use yew::{html, AttrValue, Component, Context, Html};

pub struct List {
    subscriptions: Option<Vec<SubscriptionResponse>>,
}

pub enum Msg {
    GetAllRequest,
    GetAllResponse(Result<Vec<SubscriptionResponse>, anyhow::Error>),
    DeleteRequest(u32),
    DeleteResponse(Result<(), anyhow::Error>),
}

impl List {
    fn render_table(&self, ctx: &Context<List>) -> Html {
        if let Some(subs) = &self.subscriptions {
            html! {
                <table class="tftable" border="1">
                    <thead>
                        <tr>
                            <th>{ "ID" }</th>
                            <th>{ "Description" }</th>
                            <th>{ "Type" }</th>
                            <th>{ "Traffic" }</th>
                            <th>{ "Price" }</th>
                            <th>{ "Extra Traffic Price" }</th>
                            <th>{ "Actions" }</th>
                        </tr>
                    </thead>

                    <tbody>
                        { subs.iter().map(|sub| self.render_item(ctx, sub)).collect::<Html>() }
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

    fn render_item(&self, ctx: &Context<List>, sub: &SubscriptionResponse) -> Html {
        let sub_id = sub.id;

        html! {
            <tr>
                <td>
                    <AppLink to={Route::SubscriptionDetail { id: sub.id }}>
                        { &sub.id }
                    </AppLink>
                </td>
                <td>{ &sub.description }</td>
                <td>{ &sub.subscription_type }</td>
                <td>{ format!("{} Gb/s", &sub.traffic) }</td>
                <td>{ format!("{}$", &sub.price) }</td>
                <td>{ format!("{}$", &sub.extra_traffic_price) }</td>
                <td>
                    <AppLink to={Route::SubscriptionEdit { id: sub.id }}>
                        <button class="btn-warning">
                            <MatIconButton icon="edit" />
                        </button>
                    </AppLink>

                    <button class="btn-danger" onclick={ctx.link().callback(move |_| Msg::DeleteRequest(sub_id))}>
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
        Self {
            subscriptions: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();

        match msg {
            Msg::GetAllRequest => {
                log::info!("Requesting subscriptions");

                wasm_bindgen_futures::spawn_local(async move {
                    let get_subscriptions_req =
                        Request::get("http://localhost:8000/api/subscription")
                            .header("Content-Type", "application/json");

                    let resp = get_subscriptions_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let subscriptions = resp
                                    .json::<Vec<SubscriptionResponse>>()
                                    .await
                                    .map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {:?}", err)
                                    });

                                link.send_message(Msg::GetAllResponse(subscriptions));
                            } else {
                                link.send_message(Msg::GetAllResponse(Err(anyhow::anyhow!(
                                    "Failed to get subscriptions: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            log::error!("Failed to send request: {:?}", err);
                            link.send_message(Msg::GetAllResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetAllResponse(Ok(subscriptions)) => {
                self.subscriptions = Some(subscriptions);
                true
            }
            Msg::GetAllResponse(Err(_)) => false,
            Msg::DeleteRequest(id) => {
                log::info!("Deleting subscription with id: {}", id);

                wasm_bindgen_futures::spawn_local(async move {
                    let delete_subscription_req = Request::delete(
                        format!("http://localhost:8000/api/subscription/{}", id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = delete_subscription_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 204 {
                                link.send_message(Msg::DeleteResponse(Ok(())));
                            } else {
                                link.send_message(Msg::DeleteResponse(Err(anyhow::anyhow!(
                                    "Failed to delete subscription: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::DeleteResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
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
                log::error!("Failed to delete subscription: {:?}", err);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="box">
                <h2> { "Subscriptions" } </h2>
                <h3>
                    <AppLink to={Route::SubscriptionCreate}>
                        <MatButton label="Create new subscription" icon={AttrValue::from("add")} raised=true />
                    </AppLink>
                </h3>
                { self.render_table(ctx) }
            </div>
        }
    }
}
