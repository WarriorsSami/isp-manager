use common::subscription::SubscriptionResponse;
use gloo_net::http::Request;
use material_yew::{MatCircularProgress, MatIconButton, MatList, MatListItem};
use yew::{html, Component, Context, Html};

pub struct List {
    subscriptions: Option<Vec<SubscriptionResponse>>,
}

pub enum Msg {
    MakeReq,
    Resp(Result<Vec<SubscriptionResponse>, anyhow::Error>),
}

impl List {
    fn render_list(&self) -> Html {
        if let Some(subs) = &self.subscriptions {
            html! {
                <MatList multi=true>
                    {  subs.iter().map(|sub| self.render_subscription(sub)).collect::<Html>() }
                </MatList>
            }
        } else {
            html! {
                <div>
                    <MatCircularProgress indeterminate=true />
                </div>
            }
        }
    }

    fn render_subscription(&self, sub: &SubscriptionResponse) -> Html {
        html! {
            <MatListItem>
                { format!("{}. {} {} {} Gb/s {}$ {}$", &sub.id, &sub.description, &sub.subscription_type, &sub.traffic, &sub.price, &sub.extra_traffic_price) }
                <MatIconButton icon="edit" />
                <MatIconButton icon="delete" />
            </MatListItem>
        }
    }
}

impl Component for List {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_message(Msg::MakeReq);
        Self {
            subscriptions: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeReq => {
                log::info!("Requesting subscriptions");

                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let get_subscriptions_req =
                        Request::get("http://localhost:8000/api/subscription")
                            .header("Content-Type", "application/json");

                    let resp = get_subscriptions_req.send().await;

                    match resp {
                        Ok(resp) => {
                            let subscriptions = resp.json::<Vec<SubscriptionResponse>>().await;
                            match subscriptions {
                                Ok(subscriptions) => {
                                    log::info!("Subscriptions: {:?}", subscriptions);
                                    link.send_message(Msg::Resp(Ok(subscriptions)));
                                }
                                Err(err) => {
                                    log::error!("Failed to parse response: {:?}", err);
                                    link.send_message(Msg::Resp(Err(anyhow::anyhow!(
                                        "Failed to parse response: {:?}",
                                        err
                                    ))));
                                }
                            }
                        }
                        Err(err) => {
                            log::error!("Failed to send request: {:?}", err);
                            link.send_message(Msg::Resp(Err(anyhow::anyhow!(
                                "Failed to send request: {:?}",
                                err
                            ))));
                        }
                    }
                });
                false
            }
            Msg::Resp(Ok(subscriptions)) => {
                self.subscriptions = Some(subscriptions);
                true
            }
            Msg::Resp(Err(_)) => false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h2>{ "Subscriptions" }</h2>
                { self.render_list() }
            </div>
        }
    }
}
