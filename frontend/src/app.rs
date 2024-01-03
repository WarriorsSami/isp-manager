use material_yew::{
    drawer::MatDrawerAppContent,
    top_app_bar_fixed::{MatTopAppBarNavigationIcon, MatTopAppBarTitle},
    MatDrawer, MatIconButton, MatList, MatListItem, MatTopAppBarFixed,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::{contract, customer, home, invoice, payment, subscription};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/app/subscription/list")]
    SubscriptionList,
    #[at("/app/subscription/create")]
    SubscriptionCreate,
    #[at("/app/subscription/:id/edit")]
    SubscriptionEdit { id: u32 },
    #[at("/app/subscription/:id/detail")]
    SubscriptionDetail { id: u32 },
    #[at("/app/customer/list")]
    CustomerList,
    #[at("/app/customer/create")]
    CustomerCreate,
    #[at("/app/customer/:id/edit")]
    CustomerEdit { id: u32 },
    #[at("/app/customer/:id/detail")]
    CustomerDetail { id: u32 },
    #[at("/app/contract/create")]
    ContractCreate,
    #[at("/app/contract/:id/edit")]
    ContractEdit { id: u32 },
    #[at("/app/contract/:id/detail")]
    ContractDetail { id: u32 },
    #[at("/app/invoice/create")]
    InvoiceCreate,
    #[at("/app/invoice/:id/detail")]
    InvoiceDetail { id: u32 },
    #[at("/app/payment/create")]
    PaymentCreate,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub type AppLink = Link<Route>;

pub struct App {
    /// `true` represents open; `false` represents close
    drawer_state: bool,
}

pub enum Msg {
    NavIconClick,
    Opened,
    Closed,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            drawer_state: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NavIconClick => {
                self.drawer_state = !self.drawer_state;
                true
            }
            Msg::Closed => {
                self.drawer_state = false;
                false
            }
            Msg::Opened => {
                self.drawer_state = true;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        html! {
            <BrowserRouter>
                <MatDrawer open={self.drawer_state} drawer_type="dismissible"
                    onopened={link.callback(|_| Msg::Opened)}
                    onclosed={link.callback(|_| Msg::Closed)}>

                    <div class="drawer-content">
                        <MatList>
                            <AppLink to={Route::Home}><MatListItem>{"Home"}</MatListItem></AppLink>
                            <AppLink to={Route::SubscriptionList}><MatListItem>{"Subscriptions"}</MatListItem></AppLink>
                            <AppLink to={Route::CustomerList}><MatListItem>{"Customers"}</MatListItem></AppLink>
                        </MatList>
                    </div>
                    <MatDrawerAppContent>
                        <div class="app-content">
                            <MatTopAppBarFixed onnavigationiconclick={link.callback(|_| Msg::NavIconClick)}>
                                <MatTopAppBarNavigationIcon>
                                    <MatIconButton icon="menu"></MatIconButton>
                                </MatTopAppBarNavigationIcon>

                                <MatTopAppBarTitle>
                                    <div class="app-title">
                                        <AppLink to={Route::Home}>
                                            <h1>{"ISP Manager"}</h1>
                                        </AppLink>
                                    </div>
                                </MatTopAppBarTitle>

                            </MatTopAppBarFixed>

                            <Switch<Route> render={App::switch} />
                        </div>
                    </MatDrawerAppContent>
                </MatDrawer>
            </BrowserRouter>
        }
    }
}

impl App {
    fn switch(switch: Route) -> Html {
        match switch {
            Route::Home => html! { <home::Home /> },
            Route::SubscriptionList => html! { <subscription::list::List /> },
            Route::SubscriptionCreate => html! { <subscription::create::Create /> },
            Route::SubscriptionEdit { id } => html! { <subscription::edit::Edit id={id} /> },
            Route::SubscriptionDetail { id } => html! { <subscription::detail::Detail id={id} /> },
            Route::CustomerList => html! { <customer::list::List /> },
            Route::CustomerCreate => html! { <customer::create::Create /> },
            Route::CustomerEdit { id } => html! { <customer::edit::Edit id={id} /> },
            Route::CustomerDetail { id } => html! { <customer::detail::Detail id={id} /> },
            Route::ContractCreate => html! { <contract::create::Create /> },
            Route::ContractEdit { id } => html! { <contract::edit::Edit id={id} /> },
            Route::ContractDetail { id } => html! { <contract::detail::Detail id={id} /> },
            Route::InvoiceCreate => html! { <invoice::create::Create /> },
            Route::InvoiceDetail { id } => html! { <invoice::detail::Detail id={id} /> },
            Route::PaymentCreate => html! { <payment::create::Create /> },
            Route::NotFound => html! { <div class="center"><h1>{"404 Not Found"}</h1></div> },
        }
    }
}
