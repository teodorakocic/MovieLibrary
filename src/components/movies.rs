use gloo_timers::future::TimeoutFuture;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::Properties;

pub enum Msg {
    Search,
    SortByName,
    SortByRating,
    SetSort(Sort),
}

#[derive(Eq, PartialEq, Clone)]
pub struct Movie {
    name: String,
    genre: String,
    rating: String,
    actors: String,
}

#[derive(Eq, PartialEq, Properties)]
pub struct MovieCardFind {
    movie: Rc<Movie>,
    search: Rc<String>,
}

pub struct MovieCard {
    visible: bool,
}

impl Component for MovieCard {
    type Message = ();
    type Properties = MovieCardFind;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { visible: true }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx
            .props()
            .movie
            .genre
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        if ctx
            .props()
            .movie
            .name
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        if ctx
            .props()
            .movie
            .actors
            .to_lowercase()
            .contains(&ctx.props().search.to_string().to_lowercase())
        {
            self.visible = true;
            return true;
        }
        self.visible = false;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let movie_class = match self.visible {
            true => "movie-visible",
            false => "movie-hidden",
        };
        html! {
            <div class={classes!("columns", "is-mobile", "is-gapless", "is-marginless", movie_class)}>
                <div class="column is-6">
                    <div class="columns is-gapless is-marginless movie-group">
                        <div class="column is-7 movie-name">{ctx.props().movie.name.clone()}</div>
                        <div class="column is-5 movie-genre">{ctx.props().movie.genre.clone()}</div>
                    </div>
                </div>
                <div class="column is-6">
                    <div class="columns is-gapless is-marginless">
                        <div class="column is-3 is-size-6 movie-rating">{ctx.props().movie.rating.clone()}</div>
                        <div class="column is-9 gray4 has-text-right movie-actors">{ctx.props().movie.actors.clone()}</div>
                    </div>
                </div>
            </div>
        }
    }
}

#[derive(PartialEq)]
pub enum Sort {
    ByName,
    ByRating,
}

pub struct MovieList {
    movies: Vec<Rc<Movie>>,
    search: Rc<String>,
    search_input: NodeRef,
    sort: Sort,
}

impl Component for MovieList {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        MovieList {
            movies: movies_vector(),
            search: Rc::new("".to_string()),
            search_input: NodeRef::default(),
            sort: Sort::ByRating,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        macro_rules! sort {
            ($key:ident) => {
                if self.sort != Sort::$key {
                    let ms = match *self.search == "".to_string() {
                        true => 400,
                        false => 310,
                    };
                    self.search = Rc::new("DELETE_ALL_ITEMS".to_string());
                    let link = ctx.link().clone();
                    spawn_local(async move {
                        TimeoutFuture::new(ms).await;
                        link.send_message(Msg::SetSort(Sort::$key));
                    });
                }
            };
        }
        match msg {
            Msg::Search => {
                self.search = Rc::new(
                    self.search_input
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value(),
                );
            }
            Msg::SortByName => {
                sort!(ByName);
            }
            Msg::SortByRating => {
                sort!(ByRating);
            }
            Msg::SetSort(m) => {
                self.sort = m;
                match self.sort {
                    Sort::ByName => self.movies.sort_by(|a, b| a.name.cmp(&b.name)),
                    Sort::ByRating => self.movies.sort_by(|a, b| b.rating.cmp(&a.rating)),
                }
                ctx.link().send_message(Msg::Search);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        macro_rules! show_table_head {
            ($class:expr, $msg:expr, $sort:expr, $text:expr) => {{
                html! {
                    <div class={$class}>
                        <a class="has-text-light has-text-weight-bold"
                        onclick={ctx.link().callback(|_| Msg::$msg)}>
                            {$text}
                            if self.sort == Sort::$sort {
                                {" ▾"}
                            }
                        </a>
                    </div>
                }
            }};
        }
        html! {
            <div class="container has-text-left" style="max-width: 1030px">
                <div class="has-text-centered"><input
                    class="input is-rounded is-white has-text-centered is-small is-overlay"
                    style="max-width: 250px"
                    placeholder="Type to search..."
                    type="text" ref={self.search_input.clone()}
                    onkeyup={ctx.link().callback(|_| Msg::Search)}/>
                </div>
                <br/>
                <div style="border-bottom: 2px solid #634a4d; background-color: #3d2d2f;
                            padding: 5px 15px 5px 15px; border-radius: 20px">
                  <div class="columns is-mobile is-gapless is-marginless is-clearfix">
                     <div class="column is-6"><div class="columns is-gapless is-marginless movie-group">
                        {show_table_head!("column is-7 movie-name", SortByName, ByName, "name")}
                     </div></div>
                     <div class="column is-6"><div class="columns is-gapless is-marginless">
                        {show_table_head!("column is-3 movie-rating", SortByRating, ByRating, "✫")}
                        <div class="column is-9 gray4 movie-actors">
                            <div class="has-text-weight-bold has-text-right">{"actors"}</div>
                        </div>
                     </div></div>
                  </div>
                </div>
                <div style="padding: 0px 15px 0px 15px">
                    { for self.movies.iter().map(|m|
                        html! { <MovieCard movie={m.clone()} search={self.search.clone()}/> }
                    ) }
                </div>
            </div>
        }
    }
}

fn movies_vector() -> Vec<Rc<Movie>> {
     macro_rules!
     movies_vec {
            ($({ $name:literal, $genre:literal, $rating:literal, $actors:literal },)*) => {
                vec![
                    $(
                        Rc::new(Movie {
                            name: $name.to_string(),
                            genre: $genre.to_string(),
                            rating: concat!($rating, " / 10").to_string(),
                            actors: $actors.to_string(),
                        }),
                    )*
                ]
            };
        }

    movies_vec! { 
        { "X-Men: Days of Future Past", "Action", "7.9", "Patrick Stewart, Ian McKellen, Hugh Jackman, James McAvoy" } ,
        { "The Batman", "Action", "7.9", "Robert Pattinson, Zoë Kravitz, Jeffrey Wright, Colin Farrell" } ,
        { "Edge of Tomorrow", "Action", "7.9", "Tom Cruise, Emily Blunt, Bill Paxton, Brendan Gleeson" } ,
        { "District 9", "Action", "7.9", "Sharlto Copley, David James, Jason Cope, Nathalie Boltt" } ,
        { "Star Trek", "Action", "7.9", "Chris Pine, Zachary Quinto, Simon Pegg, Leonard Nimoy" } ,
        { "Letters from Iwo Jima", "Action", "7.9", "Ken Watanabe, Kazunari Ninomiya, Tsuyoshi Ihara, Ryô Kase" } ,
        { "Iron Man", "Action", "7.9", "Robert Downey Jr., Gwyneth Paltrow, Terrence Howard, Jeff Bridges" } ,
        { "Ying xiong", "Action", "7.9", "Jet Li, Tony Chiu-Wai Leung, Maggie Cheung, Ziyi Zhang" } ,
        { "The Bourne Identity", "Action", "7.9", "Franka Potente, Matt Damon, Chris Cooper, Clive Owen" } ,
        { "The Matrix", "Action", "8.7", "Keanu Reeves, Laurence Fishburne, Carrie-Anne Moss, Hugo Weaving" } ,
        { "The Empire Strikes Back", "Action", "8.7", "Mark Hamill, Harrison Ford, Carrie Fisher, Billy Dee Williams" } ,
        { "Terminator 2: Judgment Day", "Action", "8.6", "Arnold Schwarzenegger, Linda Hamilton, Edward Furlong, Robert Patrick" } ,
        { "Star Wars", "Action", "8.6", "Mark Hamill, Harrison Ford, Carrie Fisher, Alec Guinness" } ,
        { "Seppuku", "Action", "8.6", "Tatsuya Nakadai, Akira Ishihama, Shima Iwashita, Tetsurô Tanba" } ,
        { "Shichinin no samurai", "Action", "8.6", "Toshirô Mifune, Takashi Shimura, Keiko Tsushima, Yukiko Shimazaki" } ,
        { "Kaithi", "Action", "8.5", "Karthi, Narain, Arjun Das, George Maryan" } ,
        { "Asuran", "Action", "8.5", "Dhanush, Manju Warrier, Prakash Raj, Pasupathy" } ,
        { "Sita Ramam", "Action", "8.5", "Dulquer Salmaan, Mrunal Thakur, Rashmika Mandanna, Sumanth" } ,
        { "Gladiator", "Action", "8.5", "Russell Crowe, Joaquin Phoenix, Connie Nielsen, Oliver Reed" } ,
        { "Léon", "Action", "8.5", "Jean Reno, Gary Oldman, Natalie Portman, Danny Aiello" } ,
        { "Vikram", "Action", "8.4", "Kamal Haasan, Vijay Sethupathi, Fahadh Faasil, Narain" } ,
        { "Spider-Man: Into the Spider-Verse", "Animation", "8.4", "Shameik Moore, Jake Johnson, Hailee Steinfeld, Mahershala Ali" } ,
        { "Avengers: Endgame", "Action", "8.4", "Robert Downey Jr., Chris Evans, Mark Ruffalo, Chris Hemsworth" } ,
        { "Avengers: Infinity War", "Action", "8.4", "Robert Downey Jr., Chris Hemsworth, Mark Ruffalo, Chris Evans" } ,
        { "Top Gun: Maverick", "Action", "8.4", "Tom Cruise, Jennifer Connelly, Miles Teller, Val Kilmer" } ,
        { "The Dark Knight Rises", "Action", "8.4", "Christian Bale, Tom Hardy, Anne Hathaway, Gary Oldman" } ,
        { "K.G.F: Chapter 2", "Action", "8.4", "Yash, Sanjay Dutt, Raveena Tandon, Srinidhi Shetty" } ,
        { "Shershaah", "Action", "8.4", "Sidharth Malhotra, Kiara Advani, Shiv Panditt, Pranay Pachauri" } ,
        { "Oldeuboi", "Action", "8.4", "Choi Min-sik, Yoo Ji-tae, Kang Hye-jeong, Kim Byeong-Ok" } ,
        { "Mononoke-hime", "Animation", "8.4", "Yôji Matsuda, Yuriko Ishida, Yûko Tanaka, Billy Crudup" } ,
        { "Aliens", "Action", "8.4", "Sigourney Weaver, Michael Biehn, Carrie Henn, Paul Reiser" } ,
        { "Raiders of the Lost Ark", "Action", "8.4", "Harrison Ford, Karen Allen, Paul Freeman, John Rhys-Davies" } ,
        { "Vikram Vedha", "Action", "8.3", "Madhavan, Vijay Sethupathi, Shraddha Srinath, Kathir" } ,
        { "Dangal", "Action", "8.3", "Aamir Khan, Sakshi Tanwar, Fatima Sana Shaikh, Sanya Malhotra" } ,
        { "Spider-Man: No Way Home", "Action", "8.3", "Tom Holland, Zendaya, Benedict Cumberbatch, Jacob Batalon" } ,
        { "Heat", "Action", "8.3", "Al Pacino, Robert De Niro, Val Kilmer, Jon Voight" } ,
        { "Star Wars: Episode VI - Return of the Jedi", "Action", "8.3", "Mark Hamill, Harrison Ford, Carrie Fisher, Billy Dee Williams" } ,
        { "North by Northwest", "Action", "8.3", "Cary Grant, Eva Marie Saint, James Mason, Jessie Royce Landis" } ,
        { "Major", "Action", "8.2", "Adivi Sesh, Prakash Raj, Revathi, Saiee Manjrekar" } ,
        { "1917", "Action", "8.2", "Dean-Charles Chapman, George MacKay, Daniel Mays, Colin Firth" } ,
        { "Uri: The Surgical Strike", "Action", "8.2", "Vicky Kaushal, Paresh Rawal, Mohit Raina, Yami Gautam" } ,
        { "K.G.F: Chapter 1", "Action", "8.2", "Yash, Srinidhi Shetty, Ramachandra Raju, Archana Jois" } ,
        { "Dag II", "Action", "8.2", "Caglar Ertugrul, Ufuk Bayraktar, Ahu Türkpençe, Murat Serezli" } ,
        { "Baahubali 2: The Conclusion", "Action", "8.2", "Prabhas, Rana Daggubati, Anushka Shetty, Tamannaah Bhatia" } ,
        { "Gangs of Wasseypur", "Action", "8.2", "Manoj Bajpayee, Ashish Chhipa, Richa Chadha, Nawazuddin Siddiqui" } ,
        { "Paan Singh Tomar", "Action", "8.2", "Irrfan Khan, Mahie Gill, Rajesh Abhay, Hemendra Dandotiya" } ,
        { "Warrior", "Action", "8.2", "Tom Hardy, Nick Nolte, Joel Edgerton, Jennifer Morrison" } ,
        { "Kimetsu no Yaiba: Mugen Ressha-Hen", "Animation", "8.2", "Natsuki Hanae, Akari Kitô, Yoshitsugu Matsuoka, Hiro Shimono" } ,
        { "V for Vendetta", "Action", "8.2", "Hugo Weaving, Natalie Portman, Rupert Graves, Stephen Rea" } ,
        { "Batman Begins", "Action", "8.2", "Christian Bale, Michael Caine, Ken Watanabe, Liam Neeson" } ,
        { "Kill Bill: Vol. 1", "Action", "8.2", "Uma Thurman, David Carradine, Daryl Hannah, Michael Madsen" } ,
        { "Lock, Stock and Two Smoking Barrels", "Action", "8.2", "Jason Flemyng, Dexter Fletcher, Nick Moran, Jason Statham" } ,
        { "Jurassic Park", "Action", "8.2", "Sam Neill, Laura Dern, Jeff Goldblum, Richard Attenborough" } ,
        { "Indiana Jones and the Last Crusade", "Action", "8.2", "Harrison Ford, Sean Connery, Alison Doody, Denholm Elliott" } ,
        { "Die Hard", "Action", "8.2", "Bruce Willis, Alan Rickman, Bonnie Bedelia, Reginald VelJohnson" } ,
        { "Ran", "Action", "8.2", "Tatsuya Nakadai, Akira Terao, Jinpachi Nezu, Daisuke Ryû" } ,
        { "Yôjinbô", "Action", "8.2", "Toshirô Mifune, Eijirô Tôno, Tatsuya Nakadai, Yôko Tsukasa" } ,
        { "The General", "Action", "8.2", "Buster Keaton, Marion Mack, Glen Cavender, Jim Farley" } ,
        { "Sherlock Jr.", "Action", "8.2", "Buster Keaton, Kathryn McGuire, Joe Keaton, Erwin Connelly" } ,
        { "Everything Everywhere All at Once", "Action", "8.1", "Michelle Yeoh, Stephanie Hsu, Jamie Lee Curtis, Ke Huy Quan" } ,
        { "Bajrangi Bhaijaan", "Action", "8.1", "Salman Khan, Harshaali Malhotra, Nawazuddin Siddiqui, Kareena Kapoor" } ,
        { "Logan", "Action", "8.1", "Hugh Jackman, Patrick Stewart, Dafne Keen, Boyd Holbrook" } ,
        { "Rush", "Action", "8.1", "Daniel Brühl, Chris Hemsworth, Olivia Wilde, Alexandra Maria Lara" } ,
        { "Ford v Ferrari", "Action", "8.1", "Matt Damon, Christian Bale, Jon Bernthal, Caitríona Balfe" } ,
        { "Mad Max: Fury Road", "Action", "8.1", "Tom Hardy, Charlize Theron, Nicholas Hoult, Zoë Kravitz" } ,
        { "A Wednesday", "Action", "8.1", "Anupam Kher, Naseeruddin Shah, Veerendra Saxena, Mahesh Kanual" } ,
        { "How to Train Your Dragon", "Animation", "8.1", "Jay Baruchel, Gerard Butler, Christopher Mintz-Plasse, Craig Ferguson" } ,
        { "Pirates of the Caribbean: The Curse of the Black Pearl", "Action", "8.1", "Johnny Depp, Geoffrey Rush, Orlando Bloom, Keira Knightley" } ,
        { "Hera Pheri", "Action", "8.1", "Akshay Kumar, Suniel Shetty, Paresh Rawal, Tabu" } ,
        { "Sarfarosh", "Action", "8.1", "Aamir Khan, Naseeruddin Shah, Sonali Bendre, Mukesh Rishi" } ,
        { "The Iron Giant", "Animation", "8.1", "Eli Marienthal, Harry Connick Jr., Jennifer Aniston, Vin Diesel" } ,
        { "The Terminator", "Action", "8.1", "Arnold Schwarzenegger, Linda Hamilton, Michael Biehn, Paul Winfield" } ,
        { "Blade Runner", "Action", "8.1", "Harrison Ford, Rutger Hauer, Sean Young, Edward James Olmos" } ,
        { "Sholay", "Action", "8.1", "Sanjeev Kumar, Dharmendra, Amitabh Bachchan, Amjad Khan" } ,
        { "Kakushi-toride no san-akunin", "Action", "8.1", "Toshirô Mifune, Misa Uehara, Minoru Chiaki, Kamatari Fujiwara" } ,
        { "White Heat", "Action", "8.1", "James Cagney, Virginia Mayo, Edmond O'Brien, Margaret Wycherly" } ,
        { "RRR (Rise Roar Revolt)", "Action", "8", "N.T. Rama Rao Jr., Ram Charan Teja, Ajay Devgn, Alia Bhatt" } ,
        { "Arjun Reddy", "Action", "8", "Vijay Deverakonda, Shalini Pandey, Jia Sharma, Kanchana" } ,
        { "Kaththi", "Action", "8", "Joseph Vijay, Samantha Ruth Prabhu, Neil Nitin Mukesh, Tota Roy Chowdhury" } ,
        { "Haider", "Action", "8", "Shahid Kapoor, Tabu, Shraddha Kapoor, Kay Kay Menon" } ,
        { "Bãhubali: The Beginning", "Action", "8", "Prabhas, Rana Daggubati, Anushka Shetty, Tamannaah Bhatia" } ,
        { "1 - Nenokkadine", "Action", "8", "Mahesh Babu, Kriti Sanon, Nassar, Pradeep Singh Rawat" } ,
        { "Thuppakki", "Action", "8", "Joseph Vijay, Kajal Aggarwal, Vidyut Jammwal, Sathyan" } ,
        { "Guardians of the Galaxy", "Action", "8", "Chris Pratt, Vin Diesel, Bradley Cooper, Zoe Saldana" } ,
        { "Blade Runner 2049", "Action", "8", "Harrison Ford, Ryan Gosling, Ana de Armas, Dave Bautista" } ,
        { "The Revenant", "Action", "8", "Leonardo DiCaprio, Tom Hardy, Will Poulter, Domhnall Gleeson" } ,
        { "Tropa de Elite 2: O Inimigo Agora é Outro", "Action", "8", "Wagner Moura, Irandhir Santos, André Ramiro, Milhem Cortaz" } ,
        { "Deadpool", "Action", "8", "Ryan Reynolds, Morena Baccarin, T.J. Miller, Ed Skrein" } ,
        { "Karthikeya 2", "Action", "8", "Nikhil Siddharth, Anupama Parameswaran, Srinivasa Reddy, Harsha Chemudu" } ,
        { "Zack Snyder's Justice League", "Action", "8", "Henry Cavill, Ben Affleck, Gal Gadot, Amy Adams" } ,
        { "Ip Man", "Action", "8", "Donnie Yen, Simon Yam, Siu-Wong Fan, Ka-Tung Lam" } ,
        { "Nefes: Vatan Sagolsun", "Action", "8", "Mete Horozoglu, Ilker Kizmaz, Baris Bagci, Özgür Eren Koç" } ,
        { "Dune: Part One", "Action", "8", "Timothée Chalamet, Rebecca Ferguson, Zendaya, Oscar Isaac" } ,
        { "Tropa de Elite", "Action", "8", "Wagner Moura, André Ramiro, Caio Junqueira, Milhem Cortaz" } ,
        { "The Avengers", "Action", "8", "Robert Downey Jr., Chris Evans, Scarlett Johansson, Jeremy Renner" } ,
        { "The Bourne Ultimatum", "Action", "8", "Matt Damon, Edgar Ramírez, Joan Allen, Julia Stiles" } ,
        { "Taegukgi hwinalrimyeo", "Action", "8", "Jang Dong-Gun, Won Bin, Eun-ju Lee, Hyeong-jin Kong" } ,
        { "Casino Royale", "Action", "8", "Daniel Craig, Eva Green, Judi Dench, Jeffrey Wright" } ,
        { "Kill Bill: Vol. 2", "Action", "8", "Uma Thurman, David Carradine, Michael Madsen, Daryl Hannah" } ,
        { "Memoirs of a Geisha", "Drama", "7.3", "Ziyi Zhang, Ken Watanabe, Michelle Yeoh, Suzuka Ohgo" } ,
        { "Eight Below", "Adventure", "7.3", "Paul Walker, Jason Biggs, Bruce Greenwood, Moon Bloodgood" } ,
        { "North Country", "Drama", "7.3", "Charlize Theron, Jeremy Renner, Frances McDormand, Thomas Curtis" } ,
        { "The Squid and the Whale", "Comedy", "7.3", "Owen Kline, Jeff Daniels, Laura Linney, Jesse Eisenberg" } ,
        { "The Secret Life of Walter Mitty", "Adventure", "7.3", "Ben Stiller, Kristen Wiig, Jon Daly, Kathryn Hahn" } ,
        { "Far from Heaven", "Drama", "7.3", "Julianne Moore, Dennis Quaid, Dennis Haysbert, Patricia Clarkson" } ,
        { "L'auberge espagnole", "Comedy", "7.3", "Romain Duris, Judith Godrèche, Kelly Reilly, Audrey Tautou" } ,
        { "A Walk to Remember", "Drama", "7.3", "Mandy Moore, Shane West, Peter Coyote, Daryl Hannah" } ,
        { "Punch-Drunk Love", "Comedy", "7.3", "Adam Sandler, Emily Watson, Philip Seymour Hoffman, Jason Andrews" } ,
        { "Monsoon Wedding", "Comedy", "7.3", "Naseeruddin Shah, Lillete Dubey, Shefali Shah, Vijay Raaz" } ,
        { "Chocolat", "Drama", "7.3", "Juliette Binoche, Johnny Depp, Judi Dench, Alfred Molina" } ,
        { "Finding Forrester", "Drama", "7.3", "Sean Connery, Rob Brown, F. Murray Abraham, Anna Paquin" } ,
        { "Ghost World", "Comedy", "7.3", "Steve Buscemi, Thora Birch, Scarlett Johansson, Brad Renfro" } ,
        { "10 Things I Hate About You", "Comedy", "7.3", "Heath Ledger, Julia Stiles, Joseph Gordon-Levitt, Larisa Oleynik" } ,
        { "Dogma", "Adventure", "7.3", "Ben Affleck, Matt Damon, Linda Fiorentino, Bud Cort" } ,
        { "Carne trémula", "Drama", "7.3", "Liberto Rabal, Francesca Neri, Javier Bardem, Ángela Molina" } ,
        { "Jerry Maguire", "Comedy", "7.3", "Tom Cruise, Cuba Gooding Jr., Renée Zellweger, Kelly Preston" } ,
        { "Murder in the First", "Drama", "7.3", "Christian Slater, Kevin Bacon, Gary Oldman, Embeth Davidtz" } ,
        { "Mr. Holland's Opus", "Drama", "7.3", "Richard Dreyfuss, Glenne Headly, Jay Thomas, Olympia Dukakis" } ,
        { "Little Women", "Drama", "7.3", "Susan Sarandon, Winona Ryder, Kirsten Dunst, Claire Danes" } ,
        { "The Secret Garden", "Drama", "7.3", "Kate Maberly, Maggie Smith, Heydon Prowse, Andrew Knott" } ,
        { "Much Ado About Nothing", "Comedy", "7.3", "Kenneth Branagh, Emma Thompson, Keanu Reeves, Kate Beckinsale" } ,
        { "A League of Their Own", "Comedy", "7.3", "Tom Hanks, Geena Davis, Lori Petty, Madonna" } ,
        { "Steel Magnolias", "Comedy", "7.3", "Shirley MacLaine, Olympia Dukakis, Sally Field, Julia Roberts" } ,
        { "Say Anything...", "Comedy", "7.3", "John Cusack, Ione Skye, John Mahoney, Lili Taylor" } ,
        { "Driving Miss Daisy", "Comedy", "7.3", "Morgan Freeman, Jessica Tandy, Dan Aykroyd, Patti LuPone" } ,
        { "The Unbearable Lightness of Being", "Drama", "7.3", "Daniel Day-Lewis, Juliette Binoche, Lena Olin, Derek de Lint" } ,
        { "Big", "Comedy", "7.3", "Tom Hanks, Elizabeth Perkins, Robert Loggia, John Heard" } ,
        { "The NeverEnding Story", "Adventure", "7.3", "Noah Hathaway, Barret Oliver, Tami Stronach, Gerald McRaney" } ,
        { "Possession", "Drama", "7.3", "Isabelle Adjani, Sam Neill, Margit Carstensen, Heinz Bennent" } ,
        { "Excalibur", "Adventure", "7.3", "Nigel Terry, Helen Mirren, Nicholas Clay, Cherie Lunghi" } ,
        { "Rocky II", "Drama", "7.3", "Sylvester Stallone, Talia Shire, Burt Young, Carl Weathers" } ,
        { "Slap Shot", "Comedy", "7.3", "Paul Newman, Michael Ontkean, Strother Martin, Jennifer Warren" } ,
        { "Easy Rider", "Adventure", "7.3", "Peter Fonda, Dennis Hopper, Jack Nicholson, Antonio Mendoza" } ,
        { "The French Dispatch", "Comedy", "7.2", "Benicio Del Toro, Adrien Brody, Tilda Swinton, Léa Seydoux" } ,
        { "A Quiet Place Part II", "Drama", "7.2", "Emily Blunt, Millicent Simmonds, Cillian Murphy, John Krasinski" } ,
        { "Honey Boy", "Drama", "7.2", "Shia LaBeouf, Lucas Hedges, Noah Jupe, Byron Bowers" } ,
        { "Palmer", "Drama", "7.2", "Justin Timberlake, Juno Temple, Alisha Wainwright, Ryder Allen" } ,
        { "Five Feet Apart", "Drama", "7.2", "Haley Lu Richardson, Cole Sprouse, Moises Arias, Kimberly Hebert Gregory" } ,
        { "Jolly LLB 2", "Comedy", "7.2", "Akshay Kumar, Huma Qureshi, Saurabh Shukla, Annu Kapoor" } ,
        { "Una Mujer Fantástica", "Drama", "7.2", "Daniela Vega, Francisco Reyes, Luis Gnecco, Aline Küppenheim" } ,
        { "The Square", "Comedy", "7.2", "Claes Bang, Elisabeth Moss, Dominic West, Terry Notary" } ,
        { "Christopher Robin", "Adventure", "7.2", "Ewan McGregor, Hayley Atwell, Bronte Carmichael, Mark Gatiss" } ,
        { "The Wife", "Drama", "7.2", "Glenn Close, Jonathan Pryce, Max Irons, Christian Slater" } ,
        { "T2 Trainspotting", "Drama", "7.2", "Ewan McGregor, Ewen Bremner, Jonny Lee Miller, Robert Carlyle" } ,
        { "Fences", "Drama", "7.2", "Denzel Washington, Viola Davis, Stephen McKinley Henderson, Jovan Adepo" } ,
        { "The Light Between Oceans", "Drama", "7.2", "Michael Fassbender, Alicia Vikander, Rachel Weisz, Florence Clery" } ,
        { "Carol", "Drama", "7.2", "Cate Blanchett, Rooney Mara, Sarah Paulson, Kyle Chandler" } ,
        { "Stuck in Love", "Comedy", "7.2", "Greg Kinnear, Jennifer Connelly, Lily Collins, Nat Wolff" } ,
        { "Yeh Jawaani Hai Deewani", "Drama", "7.2", "Ranbir Kapoor, Deepika Padukone, Aditya Roy Kapoor, Kalki Koechlin" } ,
        { "St. Vincent", "Comedy", "7.2", "Bill Murray, Melissa McCarthy, Naomi Watts, Jaeden Martell" } ,
        { "Beasts of the Southern Wild", "Adventure", "7.2", "Quvenzhané Wallis, Dwight Henry, Levy Easterly, Lowell Landes" } ,
        { "Turist", "Comedy", "7.2", "Johannes Kuhnke, Lisa Loven Kongsli, Clara Wettergren, Vincent Wettergren" } ,
        { "Ruby Sparks", "Comedy", "7.2", "Paul Dano, Zoe Kazan, Annette Bening, Antonio Banderas" } ,
        { "A Dog's Purpose", "Adventure", "7.2", "Josh Gad, Dennis Quaid, Peggy Lipton, Bryce Gheisar" } ,
        { "Shame", "Drama", "7.2", "Michael Fassbender, Carey Mulligan, James Badge Dale, Lucy Walters" } ,
        { "Only Lovers Left Alive", "Comedy", "7.2", "Tilda Swinton, Tom Hiddleston, Mia Wasikowska, John Hurt" } ,
        { "The Age of Adaline", "Drama", "7.2", "Blake Lively, Michiel Huisman, Harrison Ford, Kathy Baker" } ,
        { "Beginners", "Comedy", "7.2", "Ewan McGregor, Christopher Plummer, Mélanie Laurent, Goran Visnjic" } ,
        { "The Best Exotic Marigold Hotel", "Comedy", "7.2", "Judi Dench, Bill Nighy, Maggie Smith, Tom Wilkinson" } ,
        { "Kynodontas", "Drama", "7.2", "Christos Stergioglou, Michele Valley, Angeliki Papoulia, Christos Passalis" } ,
        { "The Great Gatsby", "Drama", "7.2", "Leonardo DiCaprio, Carey Mulligan, Joel Edgerton, Tobey Maguire" } ,
        { "August: Osage County", "Comedy", "7.2", "Meryl Streep, Dermot Mulroney, Julia Roberts, Juliette Lewis" } ,
        { "Crazy Heart", "Drama", "7.2", "Jeff Bridges, Maggie Gyllenhaal, Colin Farrell, James Keane" } ,
        { "Soul Kitchen", "Comedy", "7.2", "Adam Bousdoukos, Moritz Bleibtreu, Pheline Roggan, Anna Bederke" } ,
        { "Enter the Void", "Drama", "7.2", "Nathaniel Brown, Paz de la Huerta, Cyril Roy, Olly Alexander" } ,
        { "Adam", "Comedy", "7.2", "Hugh Dancy, Rose Byrne, Peter Gallagher, Amy Irving" } ,
        { "Rab Ne Bana Di Jodi", "Comedy", "7.2", "Shah Rukh Khan, Anushka Sharma, Vinay Pathak, M.K. Raina" } ,
        { "Don't Look Up", "Comedy", "7.2", "Leonardo DiCaprio, Jennifer Lawrence, Meryl Streep, Cate Blanchett" } ,
        { "Licorice Pizza", "Comedy", "7.2", "Alana Haim, Cooper Hoffman, Sean Penn, Tom Waits" } ,
        { "Los abrazos rotos", "Drama", "7.2", "Penélope Cruz, Lluís Homar, Blanca Portillo, José Luis Gómez" } ,
        { "The Road", "Drama", "7.2", "Viggo Mortensen, Charlize Theron, Kodi Smit-McPhee, Robert Duvall" } ,
        { "The Darjeeling Limited", "Adventure", "7.2", "Owen Wilson, Adrien Brody, Jason Schwartzman, Amara Karan" } ,
        { "Wristcutters: A Love Story", "Comedy", "7.2", "Patrick Fugit, Shea Whigham, Tom Waits, Will Arnett" } ,
        { "Peaceful Warrior", "Drama", "7.2", "Scott Mechlowicz, Nick Nolte, Amy Smart, Tim DeKay" } ,
        { "Candy", "Drama", "7.2", "Heath Ledger, Abbie Cornish, Geoffrey Rush, Tom Budge" } ,
        { "The Secret Life of Bees", "Drama", "7.2", "Dakota Fanning, Jennifer Hudson, Queen Latifah, Alicia Keys" } ,
        { "Me and You and Everyone We Know", "Comedy", "7.2", "John Hawkes, Miranda July, Miles Thompson, Brandon Ratcliff" } ,
        { "Paris, je t'aime", "Comedy", "7.2", "Juliette Binoche, Leonor Watling, Ludivine Sagnier, Fanny Ardant" } ,
        { "Bridge to Terabithia", "Drama", "7.2", "Josh Hutcherson, AnnaSophia Robb, Zooey Deschanel, Robert Patrick" } ,
        { "Speak", "Drama", "7.2", "Kristen Stewart, Elizabeth Perkins, Richard Hagerman, Allison Siko" } ,
        { "Closer", "Drama", "7.2", "Natalie Portman, Jude Law, Clive Owen, Julia Roberts" } ,
        { "The Woodsman", "Drama", "7.2", "Kevin Bacon, Kyra Sedgwick, Yasiin Bey, David Alan Grier" } ,
        { "La science des rêves", "Comedy", "7.2", "Gael García Bernal, Charlotte Gainsbourg, Miou-Miou, Alain Chabat" } ,
        { "The Passion of the Christ", "Drama", "7.2", "Jim Caviezel, Monica Bellucci, Maia Morgenstern, Christo Jivkov" } ,
        { "8 Mile", "Drama", "7.2", "Eminem, Brittany Murphy, Kim Basinger, Mekhi Phifer" } ,
        { "The Phantom of the Opera", "Drama", "7.2", "Gerard Butler, Emmy Rossum, Patrick Wilson, Miranda Richardson" } ,
        { "About Schmidt", "Drama", "7.2", "Jack Nicholson, Hope Davis, Dermot Mulroney, Kathy Bates" } ,
        { "Pay It Forward", "Drama", "7.2", "Kevin Spacey, Haley Joel Osment, Helen Hunt, Jay Mohr" } ,
        { "A.I. Artificial Intelligence", "Drama", "7.2", "Haley Joel Osment, Jude Law, Frances O'Connor, Sam Robards" } ,
        { "Wonder Boys", "Comedy", "7.2", "Michael Douglas, Tobey Maguire, Frances McDormand, Robert Downey Jr." } ,
        { "The Virgin Suicides", "Drama", "7.2", "Kirsten Dunst, Josh Hartnett, James Woods, Kathleen Turner" } ,
        { "Sweet and Lowdown", "Comedy", "7.2", "Sean Penn, Samantha Morton, Woody Allen, Ben Duncan" } ,
        { "Notting Hill", "Comedy", "7.2", "Hugh Grant, Julia Roberts, Richard McCabe, Rhys Ifans" } ,
        { "Meet Joe Black", "Drama", "7.2", "Brad Pitt, Anthony Hopkins, Claire Forlani, Jake Weber" } ,
        { "The Full Monty", "Comedy", "7.2", "Robert Carlyle, Tom Wilkinson, Mark Addy, Wim Snape" } ,
        { "Chasing Amy", "Comedy", "7.2", "Ben Affleck, Joey Lauren Adams, Ethan Suplee, Scott Mosier" } ,
        { "Swingers", "Comedy", "7.2", "Vince Vaughn, Heather Graham, Jon Favreau, Ron Livingston" } ,
        { "Confessions of a Shopaholic", "Comedy", "5.8", "Isla Fisher, Hugh Dancy, Krysten Ritter, Joan Cusack" } ,
        { "Monte Carlo", "Adventure", "5.8", "Selena Gomez, Leighton Meester, Katie Cassidy, Cory Monteith" } ,
        { "My Best Friend's Girl", "Comedy", "5.8", "Kate Hudson, Dane Cook, Jason Biggs, Alec Baldwin" } ,
        { "Made of Honor", "Comedy", "5.8", "Patrick Dempsey, Michelle Monaghan, Kevin McKidd, Kadeem Hardison" } ,
        { "Semi-Pro", "Comedy", "5.8", "Will Ferrell, Woody Harrelson, André 3000, Maura Tierney" } ,
        { "Ghosts of Girlfriends Past", "Comedy", "5.8", "Matthew McConaughey, Jennifer Garner, Emma Stone, Michael Douglas" } ,
        { "Black Sheep", "Comedy", "5.8", "Oliver Driver, Nathan Meister, Tammy Davis, Matthew Chamberlain" } ,
        { "Nacho Libre", "Comedy", "5.8", "Jack Black, Ana de la Reguera, Héctor Jiménez, Darius Rose" } ,
        { "The Heartbreak Kid", "Comedy", "5.8", "Ben Stiller, Michelle Monaghan, Malin Akerman, Jerry Stiller" } ,
        { "The Princess Diaries 2: Royal Engagement", "Comedy", "5.8", "Anne Hathaway, Callum Blue, Julie Andrews, Hector Elizondo" } ,
        { "Stuck on You", "Comedy", "5.8", "Matt Damon, Greg Kinnear, Eva Mendes, Cher" } ,
        { "The Prince & Me", "Comedy", "5.8", "Julia Stiles, Luke Mably, Miranda Richardson, Ben Miller" } ,
        { "Mr. Deeds", "Comedy", "5.8", "Adam Sandler, Winona Ryder, John Turturro, Allen Covert" } ,
        { "The Ringer", "Comedy", "5.8", "Johnny Knoxville, Katherine Heigl, Brian Cox, Jed Rees" } ,
        { "The New Guy", "Comedy", "5.8", "DJ Qualls, Lyle Lovett, Eliza Dushku, Zooey Deschanel" } ,
        { "Dracula: Dead and Loving It", "Comedy", "5.8", "Leslie Nielsen, Mel Brooks, Peter MacNicol, Steven Weber" } ,
        { "Encino Man", "Comedy", "5.8", "Sean Astin, Brendan Fraser, Pauly Shore, Megan Ward" } ,
        { "Barbarella", "Adventure", "5.8", "Jane Fonda, John Phillip Law, Anita Pallenberg, Milo O'Shea" } ,
        { "The Kissing Booth 2", "Comedy", "5.7", "Joey King, Joel Courtney, Jacob Elordi, Molly Ringwald" } ,
        { "The Wrong Missy", "Comedy", "5.7", "David Spade, Lauren Lapkus, Nick Swardson, Geoff Pierson" } ,
        { "Bhool Bhulaiyaa 2", "Comedy", "5.7", "Tabu, Kartik Aaryan, Kiara Advani, Rajpal Naurang Yadav" } ,
        { "Neighbors 2: Sorority Rising", "Comedy", "5.7", "Seth Rogen, Rose Byrne, Zac Efron, Chloë Grace Moretz" } ,
        { "I Give It a Year", "Comedy", "5.7", "Rose Byrne, Rafe Spall, Alex Macqueen, Stephen Merchant" } ,
        { "Rubber", "Comedy", "5.7", "Stephen Spinella, Roxane Mesquida, Wings Hauser, Jack Plotnick" } ,
        { "Playing for Keeps", "Comedy", "5.7", "Gerard Butler, Jessica Biel, Dennis Quaid, Noah Lomax" } ,
        { "You Again", "Comedy", "5.7", "Kristen Bell, Odette Annable, Sigourney Weaver, Jamie Lee Curtis" } ,
        { "Arthur", "Comedy", "5.7", "Russell Brand, Helen Mirren, Jennifer Garner, Greta Gerwig" } ,
        { "The Watch", "Comedy", "5.7", "Ben Stiller, Vince Vaughn, Jonah Hill, Billy Crudup" } ,
        { "Valentine's Day", "Comedy", "5.7", "Julia Roberts, Jamie Foxx, Anne Hathaway, Jessica Alba" } ,
        { "John Tucker Must Die", "Comedy", "5.7", "Jesse Metcalfe, Ashanti, Arielle Kebbel, Sophia Bush" } ,
        { "The Santa Clause 2", "Comedy", "5.7", "Tim Allen, Spencer Breslin, Elizabeth Mitchell, Eric Lloyd" } ,
        { "Life or Something Like It", "Comedy", "5.7", "Angelina Jolie, Edward Burns, Tony Shalhoub, Christian Kane" } ,
        { "Not Another Teen Movie", "Comedy", "5.7", "Chyler Leigh, Jaime Pressly, Chris Evans, Eric Christian Olsen" } ,
        { "America's Sweethearts", "Comedy", "5.7", "Julia Roberts, John Cusack, Billy Crystal, Catherine Zeta-Jones" } ,
        { "Bubble Boy", "Adventure", "5.7", "Jake Gyllenhaal, Swoosie Kurtz, Marley Shelton, Danny Trejo" } ,
        { "Deuce Bigalow: Male Gigolo", "Comedy", "5.7", "Rob Schneider, William Forsythe, Eddie Griffin, Arija Bareikis" } ,
        { "Good Burger", "Comedy", "5.7", "Kel Mitchell, Kenan Thompson, Sinbad, Abe Vigoda" } ,
        { "The Nutty Professor", "Comedy", "5.7", "Eddie Murphy, Jada Pinkett Smith, James Coburn, Larry Miller" } ,
        { "Jingle All the Way", "Adventure", "5.7", "Arnold Schwarzenegger, Sinbad, Phil Hartman, Rita Wilson" } ,
        { "Police Academy 2: Their First Assignment", "Comedy", "5.7", "Steve Guttenberg, Bubba Smith, David Graf, Michael Winslow" } ,
        { "I Feel Pretty", "Comedy", "5.6", "Amy Schumer, Michelle Williams, Emily Ratajkowski, Tom Hopper" } ,
        { "Night School", "Comedy", "5.6", "Kevin Hart, Tiffany Haddish, Rob Riggle, Romany Malco" } ,
        { "Dolittle", "Adventure", "5.6", "Robert Downey Jr., Antonio Banderas, Michael Sheen, Jim Broadbent" } ,
        { "A Bad Moms Christmas", "Comedy", "5.6", "Mila Kunis, Kristen Bell, Kathryn Hahn, Jay Hernandez" } ,
        { "Life of the Party", "Comedy", "5.6", "Melissa McCarthy, Matt Walsh, Molly Gordon, Ben Falcone" } ,
        { "Fist Fight", "Comedy", "5.6", "Ice Cube, Charlie Day, Tracy Morgan, Christina Hendricks" } ,
        { "Dumb and Dumber To", "Comedy", "5.6", "Jim Carrey, Jeff Daniels, Rob Riggle, Laurie Holden" } ,
        { "Wanderlust", "Comedy", "5.6", "Jennifer Aniston, Paul Rudd, Malin Akerman, Justin Theroux" } ,
        { "New Year's Eve", "Comedy", "5.6", "Sarah Jessica Parker, Jessica Biel, Ashton Kutcher, Michelle Pfeiffer" } ,
        { "The Sitter", "Comedy", "5.6", "Jonah Hill, Ari Graynor, Sam Rockwell, Max Records" } ,
        { "Bad Teacher", "Comedy", "5.6", "Cameron Diaz, Jason Segel, Justin Timberlake, Lucy Punch" } ,
        { "Fred Claus", "Comedy", "5.6", "Vince Vaughn, Paul Giamatti, Elizabeth Banks, John Michael Higgins" } ,
        { "You, Me and Dupree", "Comedy", "5.6", "Kate Hudson, Owen Wilson, Matt Dillon, Michael Douglas" } ,
        { "Good Luck Chuck", "Comedy", "5.6", "Dane Cook, Jessica Alba, Dan Fogler, Connor Price" } ,
        { "Failure to Launch", "Comedy", "5.6", "Matthew McConaughey, Sarah Jessica Parker, Kathy Bates, Terry Bradshaw" } ,
        { "Hatchet", "Comedy", "5.6", "Kane Hodder, Joel David Moore, Deon Richmond, Amara Zaragoza" } ,
        { "Kantara", "Action", "9.3", "Rishab Shetty, Sapthami Gowda, Kishore Kumar G., Achyuth Kumar" } ,
        { "Kicking & Screaming", "Comedy", "5.6", "Will Ferrell, Robert Duvall, Josh Hutcherson, Mike Ditka" } ,
        { "Win a Date with Tad Hamilton!", "Comedy", "5.6", "Kate Bosworth, Josh Duhamel, Topher Grace, Nathan Lane" } ,
        { "Daddy Day Care", "Comedy", "5.6", "Eddie Murphy, Jeff Garlin, Anjelica Huston, Steve Zahn" } ,
        { "Bringing Down the House", "Comedy", "5.6", "Steve Martin, Queen Latifah, Eugene Levy, Joan Plowright" } ,
        { "40 Days and 40 Nights", "Comedy", "5.6", "Josh Hartnett, Shannyn Sossamon, Paulo Costanzo, Adam Trese" } ,
        { "Runaway Bride", "Comedy", "5.6", "Julia Roberts, Richard Gere, Joan Cusack, Hector Elizondo" } ,
        { "Major League II", "Comedy", "5.6", "Charlie Sheen, Tom Berenger, Corbin Bernsen, Dennis Haysbert" } ,
        { "Sister Act 2: Back in the Habit", "Comedy", "5.6", "Whoopi Goldberg, Kathy Najimy, Maggie Smith, Barnard Hughes" } ,
        { "Dennis the Menace", "Comedy", "5.6", "Walter Matthau, Mason Gamble, Joan Plowright, Christopher Lloyd" } ,
        { "The Texas Chainsaw Massacre 2", "Comedy", "5.6", "Dennis Hopper, Caroline Williams, Jim Siedow, Bill Moseley" } ,
        { "Senior Year", "Comedy", "5.5", "Rebel Wilson, Angourie Rice, Mary Holland, Molly Brown" } ,
        { "Your Highness", "Adventure", "5.5", "Danny McBride, Natalie Portman, James Franco, Rasmus Hardiker" } ,
        { "That's My Boy", "Comedy", "5.5", "Adam Sandler, Andy Samberg, Leighton Meester, Susan Sarandon" } ,
        { "When in Rome", "Comedy", "5.5", "Kristen Bell, Josh Duhamel, Anjelica Huston, Danny DeVito" } ,
        { "Couples Retreat", "Comedy", "5.5", "Vince Vaughn, Malin Akerman, Jon Favreau, Jason Bateman" } ,
        { "Little Fockers", "Comedy", "5.5", "Ben Stiller, Teri Polo, Robert De Niro, Owen Wilson" } ,
        { "The House Bunny", "Comedy", "5.5", "Anna Faris, Colin Hanks, Emma Stone, Kat Dennings" } ,
        { "Because I Said So", "Comedy", "5.5", "Diane Keaton, Mandy Moore, Gabriel Macht, Tom Everett Scott" } ,
        { "Piranha 3D", "Comedy", "5.5", "Elisabeth Shue, Jerry O'Connell, Richard Dreyfuss, Ving Rhames" } ,
        { "Cheaper by the Dozen 2", "Adventure", "5.5", "Steve Martin, Bonnie Hunt, Hilary Duff, Eugene Levy" } ,
        { "RV", "Adventure", "5.5", "Robin Williams, Cheryl Hines, Kristin Chenoweth, JoJo" } ,
        { "The Benchwarmers", "Comedy", "5.5", "David Spade, Jon Heder, Rob Schneider, Jon Lovitz" } ,
        { "Employee of the Month", "Comedy", "5.5", "Jessica Simpson, Dane Cook, Dax Shepard, Andy Dick" } ,
        { "Monster-in-Law", "Comedy", "5.5", "Jennifer Lopez, Michael Vartan, Jane Fonda, Wanda Sykes" } ,
        { "The Lizzie McGuire Movie", "Adventure", "5.5", "Hilary Duff, Adam Lamberg, Clayton Snyder, Hallie Todd" } ,
        { "Scary Movie 3", "Comedy", "5.5", "Anna Faris, Charlie Sheen, Regina Hall, Pamela Anderson" } ,
        { "The Hot Chick", "Comedy", "5.5", "Rob Schneider, Rachel McAdams, Anna Faris, Matthew Lawrence" } ,
        { "Big Fat Liar", "Adventure", "5.5", "Frankie Muniz, Amanda Bynes, Paul Giamatti, Amanda Detmer" } ,
        { "Bride of Chucky", "Comedy", "5.5", "Jennifer Tilly, Brad Dourif, Katherine Heigl, Nick Stabile" } ,
        { "Forces of Nature", "Comedy", "5.5", "Sandra Bullock, Ben Affleck, Maura Tierney, Steve Zahn" } ,
        { "Nine Months", "Comedy", "5.5", "Hugh Grant, Julianne Moore, Tom Arnold, Joan Cusack" } ,
        { "The Dead Don't Die", "Comedy", "5.4", "Bill Murray, Adam Driver, Tom Waits, Chloë Sevigny" } ,
        { "The Boss", "Comedy", "5.4", "Melissa McCarthy, Kristen Bell, Peter Dinklage, Ella Anderson" } ,
        { "Jennifer's Body", "Comedy", "5.4", "Megan Fox, Amanda Seyfried, Adam Brody, Johnny Simmons" } ,
        { "Bride Wars", "Comedy", "5.4", "Kate Hudson, Anne Hathaway, Candice Bergen, Bryan Greenberg" } ,
        { "Teeth", "Comedy", "5.4", "Jess Weixler, John Hensley, Josh Pais, Hale Appleman" } ,
        { "Yours, Mine & Ours", "Comedy", "5.4", "Dennis Quaid, Rene Russo, Jerry O'Connell, Sean Faris" } ,
        { "Evan Almighty", "Comedy", "5.4", "Steve Carell, Morgan Freeman, Lauren Graham, Johnny Simmons" } ,
        { "Just My Luck", "Comedy", "5.4", "Lindsay Lohan, Chris Pine, Samaire Armstrong, Bree Turner" } ,
        { "Christmas with the Kranks", "Comedy", "5.4", "Tim Allen, Jamie Lee Curtis, Dan Aykroyd, M. Emmet Walsh" } ,
        { "The Perfect Man", "Comedy", "5.4", "Hilary Duff, Heather Locklear, Aria Wallace, Chris Noth" } ,
        { "Just Married", "Comedy", "5.4", "Ashton Kutcher, Brittany Murphy, Christian Kane, David Moscow" } ,
        { "Doctor Dolittle", "Comedy", "5.4", "Eddie Murphy, Peter Boyle, Ossie Davis, Oliver Platt" } ,
        { "Ri¢hie Ri¢h", "Comedy", "5.4", "Macaulay Culkin, Edward Herrmann, John Larroquette, Christine Ebersole" } ,
        { "Coneheads", "Comedy", "5.4", "Dan Aykroyd, Jane Curtin, Robert Knott, Jonathan Penner" } ,
        { "Problem Child", "Comedy", "5.4", "Michael Oliver, John Ritter, Jack Warden, Gilbert Gottfried" } ,
        { "My Stepmother Is an Alien", "Comedy", "5.4", "Dan Aykroyd, Kim Basinger, Jon Lovitz, Alyson Hannigan" } ,
        { "What Men Want", "Comedy", "5.3", "Taraji P. Henson, Kristen Ledlow, Josh Brener, Kellan Lutz" } ,
        { "Coming 2 America", "Comedy", "5.3", "Eddie Murphy, Arsenio Hall, Shari Headley, Jermaine Fowler" } ,
        { "Tusk", "Comedy", "5.3", "Justin Long, Michael Parks, Haley Joel Osment, Genesis Rodriguez" } ,
        { "Grown Ups 2", "Comedy", "5.3", "Adam Sandler, Kevin James, Chris Rock, David Spade" } ,
        { "Bachelorette", "Comedy", "5.3", "Kirsten Dunst, Isla Fisher, Lizzy Caplan, James Marsden" } ,
        { "The Back-up Plan", "Comedy", "5.3", "Jennifer Lopez, Alex O'Loughlin, Michaela Watkins, Eric Christian Olsen" } ,
        { "I Love You, Beth Cooper", "Comedy", "5.3", "Hayden Panettiere, Paul Rust, Jack Carpenter, Lauren London" } ,
        { "Old Dogs", "Comedy", "5.3", "Robin Williams, John Travolta, Seth Green, Kelly Preston" } ,
        { "Aquamarine", "Comedy", "5.3", "Emma Roberts, JoJo, Sara Paxton, Jake McDorman" } ,
        { "The Stepford Wives", "Comedy", "5.3", "Nicole Kidman, Bette Midler, Matthew Broderick, Glenn Close" } ,
        { "View from the Top", "Comedy", "5.3", "Gwyneth Paltrow, Christina Applegate, Kelly Preston, Mark Ruffalo" } ,
        { "Scary Movie 2", "Comedy", "5.3", "Anna Faris, Marlon Wayans, Antony Acker, Mark Barrett" } ,
        { "Loser", "Comedy", "5.3", "Jason Biggs, Mena Suvari, Zak Orth, Thomas Sadoski" } ,
        { "The Wedding Planner", "Comedy", "5.3", "Jennifer Lopez, Matthew McConaughey, Bridgette Wilson-Sampras, Justin Chambers" } ,
        { "Little Nicky", "Comedy", "5.3", "Adam Sandler, Patricia Arquette, Harvey Keitel, Rhys Ifans" } ,
        { "Flubber", "Comedy", "5.3", "Robin Williams, Marcia Gay Harden, Christopher McDonald, Ted Levine" } ,
        { "Police Academy 3: Back in Training", "Comedy", "5.3", "Steve Guttenberg, Bubba Smith, David Graf, Michael Winslow" } ,
        { "Popeye", "Adventure", "5.3", "Robin Williams, Shelley Duvall, Ray Walston, Paul Dooley" } ,
        { "Rough Night", "Comedy", "5.2", "Scarlett Johansson, Kate McKinnon, Zoë Kravitz, Ilana Glazer" } ,
        { "Zookeeper", "Comedy", "5.2", "Kevin James, Rosario Dawson, Leslie Bibb, Ken Jeong" } ,
        { "Beast", "Action", "5.2", "Joseph Vijay, Pooja Hegde, K. Selvaraghavan, Shine Tom Chacko" } ,
        { "License to Wed", "Comedy", "5.2", "Mandy Moore, John Krasinski, Robin Williams, Eric Christian Olsen" } ,
        { "My Super Ex-Girlfriend", "Comedy", "5.2", "Uma Thurman, Luke Wilson, Anna Faris, Rainn Wilson" } ,
        { "Astérix aux jeux olympiques", "Adventure", "5.2", "Gérard Depardieu, Clovis Cornillac, Benoît Poelvoorde, Alain Delon" } ,
        { "Mr. Woodcock", "Comedy", "5.2", "Billy Bob Thornton, Seann William Scott, Susan Sarandon, Amy Poehler" } ,
        { "The Medallion", "Action", "5.2", "Jackie Chan, Lee Evans, Claire Forlani, Julian Sands" } ,
        { "Spy Kids 2: Island of Lost Dreams", "Action", "5.2", "Alexa PenaVega, Daryl Sabara, Antonio Banderas, Carla Gugino" } ,
        { "Snow Dogs", "Adventure", "5.2", "Cuba Gooding Jr., James Coburn, Sisqó, Nichelle Nichols" } ,
        { "Sex Tape", "Comedy", "5.1", "Jason Segel, Cameron Diaz, Rob Corddry, Ellie Kemper" } ,
        { "Meet Dave", "Adventure", "5.1", "Eddie Murphy, Elizabeth Banks, Gabrielle Union, Scott Caan" } ,
        { "The Three Stooges", "Comedy", "5.1", "Sean Hayes, Chris Diamantopoulos, Will Sasso, Jane Lynch" } ,
        { "The Dukes of Hazzard", "Comedy", "5.1", "Seann William Scott, Johnny Knoxville, Jessica Simpson, Alice Greczyn" } ,
        { "Scary Movie 4", "Comedy", "5.1", "Anna Faris, Regina Hall, Craig Bierko, Bill Pullman" } ,
        { "The Sweetest Thing", "Comedy", "5.1", "Cameron Diaz, Thomas Jane, Christina Applegate, Lillian Adams" } ,
        { "Cats & Dogs", "Action", "5.1", "Alec Baldwin, Tobey Maguire, Jeff Goldblum, Elizabeth Perkins" } ,
        { "An American Werewolf in Paris", "Comedy", "5.1", "Tom Everett Scott, Julie Delpy, Vince Vieluf, Phil Buckman" } ,
        { "A Haunted House", "Comedy", "5", "Marlon Wayans, Essence Atkins, Marlene Forte, David Koechner" } ,
        { "Me Time", "Comedy", "5", "Kevin Hart, Mark Wahlberg, Regina Hall, Che Tafari" } ,
        { "Tooth Fairy", "Comedy", "5", "Dwayne Johnson, Ashley Judd, Julie Andrews, Stephen Merchant" } ,
        { "Deck the Halls", "Comedy", "5", "Matthew Broderick, Danny DeVito, Kristin Chenoweth, Kristin Davis" } ,
        { "Cursed", "Comedy", "5", "Christina Ricci, Jesse Eisenberg, Portia de Rossi, Mya" } ,
        { "The Phantom", "Action", "5", "Billy Zane, Kristy Swanson, Treat Williams, Catherine Zeta-Jones" } ,
        { "Nothing But Trouble", "Comedy", "5", "Chevy Chase, Dan Aykroyd, John Candy, Demi Moore" } ,
        { "Superman III", "Action", "5", "Christopher Reeve, Richard Pryor, Margot Kidder, Jackie Cooper" } ,
    }
}
