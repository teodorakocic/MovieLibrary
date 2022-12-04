use yew::prelude::*;
use crate::components::movies::MovieList;

pub struct MoviesTab;
impl Component for MoviesTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in">
                <div class="subtitle is-4 has-text-centered">
                    {"Movies"}
                </div>
                <MovieList/>
            </div>
        }
    }
}
