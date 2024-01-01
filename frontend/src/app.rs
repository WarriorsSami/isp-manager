use material_yew::{
    drawer::MatDrawerAppContent,
    top_app_bar_fixed::{MatTopAppBarNavigationIcon, MatTopAppBarTitle},
    MatDrawer, MatIconButton, MatList, MatListItem, MatTopAppBarFixed,
};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::home::Home;
use crate::views::subscription::create::Create;
use crate::views::subscription::edit::Edit;
use crate::views::subscription::list::List;

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
            Route::Home => html! { <Home /> },
            Route::SubscriptionList => html! { <List /> },
            Route::SubscriptionCreate => html! { <Create /> },
            Route::SubscriptionEdit { id } => html! { <Edit id={id} /> },
            Route::NotFound => html! { <h1>{"404"}</h1> },
        }
    }
}
