use yew::prelude::*;
use crate::{Route, Link};

pub struct HomeTab;
impl Component for HomeTab {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div class="container fade-in" style="margin-top: -27px">
            <div class="columns is-vcentered is-gapless">
                <div class="column is-7 has-text-centered homecolumnleft">
                    <div class="container" style="max-width: 450px">
                    <br/>
                    <div class="title is-4">
                        {"Movie Library"}
                    </div>
                    <div class="subtitle is-5 is-italic">
                        {"Lite twist on popular International Movie Database"}
                    </div>
                    <p>{"On this website You can gather the information about hunderds of movies. Also keep up with current trends in movies' world or see how famous critis have awarded the latest work of the creators of movies."}</p>
                    <div class="container" >
                        <table class="container">
                            <tr class="table-row">
                                <td class="table-cell">
                                    <Link<Route> to={Route::MoviesTab}>
                                        <button class="button button-list"></button>
                                    </Link<Route>>
                                </td>
                                <td class="table-cell">
                                    <Link<Route> to={Route::PopularTab}>
                                        <button class="button button-popular"></button>
                                    </Link<Route>>
                                </td>
                            </tr>
                            <tr class="table-row">
                                <td class="table-cell">
                                    <Link<Route> to={Route::AwardTab}>
                                        <button class="button button-award"></button>
                                    </Link<Route>>
                                </td>
                                <td class="table-cell">
                                    <Link<Route> to={Route::AboutTab}>
                                        <button class="button button-about"></button>
                                    </Link<Route>>
                                </td>
                            </tr>
                        </table>
                    </div>
                </div>
                </div>
                <div class="column">
                    <img class="imagehomeup" src="https://img.freepik.com/free-photo/young-women-with-movie-film-reel_53876-146444.jpg?size=626&ext=jpg&uid=R76159271&ga=GA1.2.1864030745.1669897465&semt=sph"/>
                    <img class="imagehomedown" src="https://img.freepik.com/premium-photo/blank-cinema-screen_1048-15981.jpg?size=626&ext=jpg&uid=R76159271&ga=GA1.2.1864030745.1669897465"/>
                </div>
            </div>
        </div>
        }
    }
}
