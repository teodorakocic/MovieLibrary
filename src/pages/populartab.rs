use crate::components::popular::{Popular, PopularCard};
use yew::prelude::*;

const CINEGRAND_URI: &str = "http://nis.cinegrand-mcf.rs/na-repertoaru-danas";

pub struct PopularTab;
impl Component for PopularTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 1200px">
                <div class="title is-4 container has-text-centered">
                    {"Currently popular"}
                    <div class="populartext" style="font-size: 14px">
                        <a href={CINEGRAND_URI}>{"repertoire"}</a>
                    </div>
                </div>
                < PopularCard popular={Popular::MoonageDaydream} />
                < PopularCard popular={Popular::TheMenu} />
                < PopularCard popular={Popular::ViolentNight} />
                < PopularCard popular={Popular::WakandaForever} />
                < PopularCard popular={Popular::PokerFace} />
            </div>
        }
    }
}
