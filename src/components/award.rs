use yew::prelude::*;

#[derive(PartialEq)]
pub enum Award {
    Dune,
    KingRichard,
    Coda,
    BeautifulMind,
    ShapeWater,
    Parasite,
    Belfast
}

pub struct AwardBinding {
    title: String,
    description: String,
    won: String,
    trailer: String,
    img: String,
}

impl From<&Award> for AwardBinding {
    fn from(a: &Award) -> Self {
        macro_rules! match_p {
            ( $( $award_key:ident => {
                    $title:expr, $file: expr, $trailer: expr, $img: expr
            }, )+
            ) => { match a {
                $( Award::$award_key => AwardBinding {
                    title: $title.to_string(),
                    description: include_str!(concat!("data/", $file, "_about.txt")).to_string(),
                    won: include_str!(concat!("data/", $file, "_won.txt")).to_string(),
                    trailer: $trailer.to_string(),
                    img: $img.to_string(),
                }, )+
            }  };
        }
        match_p!(
            Dune => { "Dune", "dune", "https://www.youtube.com/watch?v=8g18jFHCLXk",
            "https://beebom.com/wp-content/uploads/2022/04/Dune.jpg?quality=75&strip=all"
            },
            KingRichard => { "King Richard", "king", "https://www.youtube.com/watch?v=BKP_0z52ZAw",
            "https://beebom.com/wp-content/uploads/2022/04/King-Richard.jpg?quality=75&strip=all"
            },
            Coda => { "Coda", "coda", "https://www.youtube.com/watch?v=0pmfrE1YL4I",
            "https://beebom.com/wp-content/uploads/2022/04/Coda.jpg?quality=75&strip=all"
            },
            BeautifulMind => { "A beautiful mind", "mind", "https://www.youtube.com/watch?v=aS_d0Ayjw4o",
            "https://beebom.com/wp-content/uploads/2022/04/A-Beautiful-Mind.jpg?quality=75&strip=all"
            },
            ShapeWater => { "The shape of water", "water", "https://www.youtube.com/watch?v=XFYWazblaUA",
            "https://beebom.com/wp-content/uploads/2022/04/The-Shape-of-Water.jpg?quality=75&strip=all"
            },
            Parasite => { "Parasite", "parasite", "https://www.youtube.com/watch?v=5xH0HfJHsaY",
            "https://beebom.com/wp-content/uploads/2022/04/Parasite.jpg?quality=75&strip=all"
            },
            Belfast => { "Belfast", "belfast", "https://www.youtube.com/watch?v=Ja3PPOnJQ2k",
            "https://beebom.com/wp-content/uploads/2022/04/Belfast.jpg?quality=75&strip=all"
            },
        )
    }
}

pub struct AwardCard {
    award: AwardBinding,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub award: Award,
}

impl Component for AwardCard {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            award: AwardBinding::from(&ctx.props().award),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <button class="box has-text-centered awardbox">
                <div class="columns">
                    <div class="column">
                        <div class="subtitle is-4">
                            {&self.award.title}
                        </div>
                        <p class="subtitle is-italic is-6">
                            {&self.award.description}
                        </p>
                        <div class="columns has-text-centered is-mobile">
                            <div class="column">
                                <p class="subtitle is-6">
                                    {&self.award.won}
                                </p>
                            </div>
                            <div class="center">
                                <a target="_blank" href={format!("{}", &self.award.trailer)}>
                                    <button class="button is-white is-small is-outlined fixed">
                                        {"watch trailer"}
                                    </button>
                                </a>
                            </div>
                        </div>
                    </div>
                    <div class="column">
                        <img class="awardedimg" src={format!("{}", &self.award.img)}/>
                    </div>
                </div>
            </button>
        }
    }
}
