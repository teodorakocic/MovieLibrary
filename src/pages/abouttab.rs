use yew::prelude::*;

pub struct AboutTab;
impl Component for AboutTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container fade-in" style="max-width: 600px; height: 90vh">
                <div class="columns is-vcentered" style="min-height: 80vh">
                    <div class="column">
                        <div class="columns is-mobile flyfromtop">
                            <div class="column is-4"></div>
                            <div class="column">
                                <div class="subtitle is-4 flyfromright">
                                    {"General Information"}
                                </div>
                            </div>
                        </div>
                        <div class="columns is-mobile flyfromtop">
                            <div class="column is-4 has-text-right flyfromleft">{"about project:"}</div>
                            <div class="column flyfromright">{"This is the first project for the Master's course named 'Advanced Web Technologies'. This project consists of front-end application written in "}
                                <a target="_blank" href="https://doc.rust-lang.org/stable/rust-by-example/index.html">{"Rust"}</a>
                                <div class="subtitle is-7" style="margin-top: 10px">
                                    {""}
                                </div>
                            </div>
                        </div>
                        <div class="columns is-mobile flyfromtop">
                            <div class="column is-4 has-text-right flyfromleft">{"code:"}</div>
                            <div class="column flyfromright">{"Source code for this application can be viewed at - "}
                                <a target="_blank" href="https://github.com/teodorakocic/MovieLibrary">{"github"}</a>
                                <div class="subtitle is-7" style="margin-top: 10px">
                                    {""}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
