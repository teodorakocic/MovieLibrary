use yew::prelude::*;
use crate::components::award::{Award, AwardCard};

pub struct AwardTab;
impl Component for AwardTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 1000px">
               <div class="subtitle is-4 has-text-centered">
                    {"Movies with awards"}
               </div>
               < AwardCard award={Award::Dune} />
               < AwardCard award={Award::KingRichard} />
               < AwardCard award={Award::Coda} />
               < AwardCard award={Award::BeautifulMind} />
               < AwardCard award={Award::ShapeWater} />
               < AwardCard award={Award::Parasite} />
               < AwardCard award={Award::Belfast} />
            </div>
        }
    }
}
