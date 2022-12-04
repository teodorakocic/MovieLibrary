use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Popular {
    MoonageDaydream,
    TheMenu,
    ViolentNight,
    WakandaForever,
    PokerFace,
}

struct PopularBinding {
    title: String,
    genre: String,
    runtime: String,
    description: String,
    actors: String,
    gross: String,
}

impl From<&Popular> for PopularBinding {
    fn from(p: &Popular) -> Self {
        macro_rules! match_p {
            ( $( $popular_key:ident => {
                    $title:expr, $genre:expr, $runtime: expr, $file:expr
                }, )+
            ) => { match p {
                $( Popular::$popular_key => PopularBinding {
                    title: $title.to_string(),
                    genre: $genre.to_string(),
                    runtime: $runtime.to_string(),
                    description: include_str!(concat!("data/", $file, "_desc.html")).to_string(),
                    actors: include_str!(concat!("data/", $file, "_cast.html")).to_string(),
                    gross: include_str!(concat!("data/", $file, "_gross.html")).to_string(),
                }, )+
            }  };
        }
        match_p!(
            MoonageDaydream => { "Moonage Daydream", "Documentary, Music", "134", "moonage" },
            TheMenu => { "The Menu", "Horor, Mistery, Comedy", "106", "menu" },
            ViolentNight => { "Violent Night", "Action, Comedy", "101", "violent" },
            WakandaForever => { "Black Panther: Wakandra Forever", "Action, Adventure", "161", "wakanda" },
            PokerFace => { "Poker Face", "Thriller", "120", "poker" },
        )
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum TextVisibility {
    None,
    Description,
    Actors,
    Gross,
}

impl TextVisibility {
    fn to_string(&self) -> String {
        match self {
            TextVisibility::None => String::new(),
            TextVisibility::Description => "description".to_string(),
            TextVisibility::Actors => "actors".to_string(),
            TextVisibility::Gross => "gross".to_string(),
        }
    }
}

pub enum Msg {
    Buttons,
    Text(TextVisibility),
}

impl TextVisibility {
    fn show_desc(&self) -> bool {
        match self {
            TextVisibility::Description => true,
            _ => false,
        }
    }
    fn show_cast(&self) -> bool {
        match self {
            TextVisibility::Actors => true,
            _ => false,
        }
    }
    fn show_gross(&self) -> bool {
        match self {
            TextVisibility::Gross => true,
            _ => false,
        }
    }
}

pub struct PopularCard {
    popular: PopularBinding,
    text: TextVisibility,
    menu: bool,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub popular: Popular,
}

impl Component for PopularCard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            popular: PopularBinding::from(&ctx.props().popular),
            text: TextVisibility::None,
            menu: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Text(text_visibility) => {
                self.text = match text_visibility == self.text {
                    true => TextVisibility::None,
                    false => match self.text {
                        TextVisibility::None => text_visibility,
                        _ => {
                            let link = ctx.link().clone();
                            spawn_local(async move {
                                TimeoutFuture::new(1000).await;
                                link.send_message(Msg::Text(text_visibility.clone()));
                            });
                            TextVisibility::None
                        }
                    },
                };
            }
            Msg::Buttons => self.menu = !self.menu,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let desc_div = gloo_utils::document().create_element("div").unwrap();
        desc_div.set_inner_html(&self.popular.description);
        let cast_div = gloo_utils::document().create_element("div").unwrap();
        cast_div.set_inner_html(&self.popular.actors);
        let gross_div = gloo_utils::document().create_element("div").unwrap();
        gross_div.set_inner_html(&self.popular.gross);

        let menu_class = match self.menu {
            true => "popular-visible",
            false => "popular-hidden",
        };

        let menu_button_weight = |v: TextVisibility| match self.text == v {
            true => "popularbuttonbolder",
            false => "popularbutton",
        };

        fn text_class(active: bool) -> &'static str {
            match active {
                true => "populartext-visible",
                false => "populartext-hidden",
            }
        }

        macro_rules! show_button {
            ($title:expr) => {{
                html! {
                    <div class="column is-4 has-text-centered">
                        <button style="max-width: 100px; height: 28px;"
                            class={classes!("button", "is-white", "is-outlined",
                            menu_button_weight(TextVisibility::$title))}
                            onclick={ctx.link().callback(|_| Msg::Text(TextVisibility::$title))}>
                        {TextVisibility::$title.to_string()}
                        </button>
                    </div>
                }
            }};
        }

        macro_rules! show_text {
            ($title:expr, $func:expr, $div:expr) => {{
                html! {
                    <div class={classes!("populartext", text_class(self.text.$func && self.menu))}>
                        {Html::VRef($div.into())}
                    </div>
                }
            }};
        }

        html! {
            <div class="box populardiv">
                <div class="columns is-mobile is-centered">
                    <div class="column is-5">
                        <div class="title is-5">
                            {&self.popular.title}
                        </div>
                        <div class="subtitle is-6">
                            {&self.popular.genre}
                        </div>
                    </div>
                    <div class="column is-2 has-text-centered">
                        <div class="button is-white is-outlined popularbutton"
                            style="height: 26px; width: 28px; padding: 4px 0px 0px 0px; border: 0px;"
                            onclick={ctx.link().callback(|_| Msg::Buttons)}>
                            if !self.menu { { "▼" } } else { { "▲" } }
                        </div>
                    </div>
                    <div class="column is-5 has-text-right">
                        <div class="title is-5">
                            {&self.popular.runtime} {" min"}
                        </div>
                    </div>
                </div>
                <div class={classes!("columns", "is-centered", "is-mobile", menu_class)}>
                    {show_button!(Description)}
                    {show_button!(Actors)}
                    {show_button!(Gross)}
                </div>
                {show_text!(Description, show_desc(), desc_div)}
                {show_text!(Actors, show_cast(), cast_div)}
                {show_text!(Gross, show_gross(), gross_div)}
            </div>
        }
    }
}
