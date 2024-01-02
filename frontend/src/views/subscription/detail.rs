use crate::app::{AppLink, Route};
use common::subscription::SubscriptionResponse;
use gloo_net::http::Request;
use material_yew::{MatCircularProgress, MatIconButton};
use yew::{html, Component, Context, Html, Properties};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DetailProps {
    pub id: u32,
}

pub struct Detail {
    subscription: Option<SubscriptionResponse>,
}

pub enum Msg {
    GetRequest,
    GetResponse(Result<SubscriptionResponse, anyhow::Error>),
    DeleteRequest,
    DeleteResponse(Result<(), anyhow::Error>),
}

impl Detail {
    fn render_subscription(&self, ctx: &Context<Detail>) -> Html {
        if let Some(subscription) = &self.subscription {
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
                         <tr>
                            <td>{ &subscription.id }</td>
                            <td>{ &subscription.description }</td>
                            <td>{ &subscription.subscription_type }</td>
                            <td>{ format!("{} Gb/s", &subscription.traffic) }</td>
                            <td>{ format!("{}$", &subscription.price) }</td>
                            <td>{ format!("{}$", &subscription.extra_traffic_price) }</td>
                            <td>
                                <AppLink to={Route::SubscriptionEdit { id: subscription.id }}>
                                    <button class="btn-warning">
                                        <MatIconButton icon="edit" />
                                    </button>
                                </AppLink>

                                <button class="btn-danger" onclick={ctx.link().callback(move |_| Msg::DeleteRequest)}>
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
}

impl Component for Detail {
    type Message = Msg;
    type Properties = DetailProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::GetRequest);

        Self { subscription: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let link = ctx.link().clone();
        let props = ctx.props().clone();

        match msg {
            Msg::GetRequest => {
                log::info!("Fetching subscription with id: {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let get_subscription_req = Request::get(
                        format!("http://localhost:8000/api/subscription/{}", props.id).as_str(),
                    )
                    .header("Content-Type", "application/json");

                    let resp = get_subscription_req.send().await;

                    match resp {
                        Ok(resp) => {
                            if resp.status() == 200 {
                                let subscription =
                                    resp.json::<SubscriptionResponse>().await.map_err(|err| {
                                        anyhow::anyhow!("Failed to parse response: {:?}", err)
                                    });

                                link.send_message(Msg::GetResponse(subscription));
                            } else {
                                link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                    "Failed to get subscription: {:?}",
                                    resp
                                ))));
                            }
                        }
                        Err(err) => {
                            link.send_message(Msg::GetResponse(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::GetResponse(Ok(subscription)) => {
                self.subscription = Some(subscription);
                true
            }
            Msg::GetResponse(Err(err)) => {
                log::error!("Error: {:?}", err);
                false
            }
            Msg::DeleteRequest => {
                log::info!("Deleting subscription with id: {}", props.id);

                wasm_bindgen_futures::spawn_local(async move {
                    let delete_subscription_req = Request::delete(
                        format!("http://localhost:8000/api/subscription/{}", props.id).as_str(),
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
                link.navigator().unwrap().push(&Route::SubscriptionList);
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
                <h2>{ "Subscription details" }</h2>

                { self.render_subscription(ctx) }
            </div>
        }
    }
}
