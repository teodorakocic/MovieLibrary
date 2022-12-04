mod pages;
mod components;
use yew::prelude::*;
use yew_router::prelude::*;
use yew::html::Scope;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    HomeTab,
    #[at("/moviestab")]
    MoviesTab,
    #[at("/populartab")]
    PopularTab,
    #[at("/awardtab")]
    AwardTab,
    #[at("/abouttab")]
    AboutTab,
}

pub enum Msg {
    ToggleNavbar,
}
pub struct Model {
    navbar_active: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }
                <main style="margin-top: 81px; width: 100vw">
                    <Switch<Route> render={Switch::render(switch)} />
                </main>
            </BrowserRouter>
        }
    }
}

impl Model {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active, .. } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-fixed-top"
                style="background-color: #513d3f; border-width: 0px 0px 2px 0px; border-color: #3d2d2f"
                role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-5">
                        { "Movie Library" }
                    </h1>
                    <button class={classes!("navbar-burger", "burger", "button", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}
                        onclick={link.callback(|_| Msg::ToggleNavbar)}>
                    <div class="navbar-end">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::HomeTab}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::MoviesTab}>
                            { "Library" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::PopularTab}>
                            { "Popular" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::AwardTab}>
                            { "Awards" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::AboutTab}>
                            { "About" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::HomeTab => html! { <pages::hometab::HomeTab /> },
        Route::MoviesTab => html! { <pages::moviestab::MoviesTab /> },
        Route::PopularTab => html! { <pages::populartab::PopularTab /> },
        Route::AwardTab => html! { <pages::awardtab::AwardTab /> },
        Route::AboutTab => html! { <pages::abouttab::AboutTab /> },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    yew::start_app::<Model>();
}
