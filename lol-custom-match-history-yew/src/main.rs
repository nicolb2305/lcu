use crate::api::{ChampionStats, LolMatchHistoryMatchHistoryGame, Summoner};
use gloo_net::http::Request;
use icu_collator::{Collator, CollatorOptions};
use icu_locid::locale;
use icu_provider::DataLocale;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

mod api;

#[derive(Properties, PartialEq)]
pub struct SummonerProps {
    pub summoners: Vec<Summoner>,
}

#[derive(Properties, PartialEq)]
pub struct SummonerDetailsProps {
    pub summoner_id: u64,
}

#[derive(Properties, PartialEq)]
pub struct SummonerSelectorProps {
    pub on_change: Callback<u64>,
}

#[derive(Properties, PartialEq)]
pub struct Title {
    pub title: String,
}

#[derive(Properties, PartialEq)]
pub struct StatsProp {
    pub stat: ChampionStats,
}

#[function_component(Stat)]
fn stat(StatsProp { stat }: &StatsProp) -> Html {
    let games = stat.wins + stat.losses;
    let winrate = (stat.wins * 100) / games;
    let games_text = if games == 1 { "game" } else { "games" };
    let kda = (stat.kills + stat.assists) / stat.deaths;
    let square_url = format!(
        "https://cdn.communitydragon.org/latest/champion/{}/square",
        stat.champion_id
    );

    html! {
        <article class="champion-stat" style={format!("--winrate: {winrate}")}>
            <img class="stats-icon" width="48" src={square_url} />
            <div class="stats">{format!("{winrate}% ({games} {games_text})")}</div>
            <div>{format!("{} / {} / {} ({kda:.2} KDA)", stat.kills, stat.deaths, stat.assists)}</div>
        </article>
    }
}

#[function_component(Stats)]
fn stats(SummonerDetailsProps { summoner_id }: &SummonerDetailsProps) -> Html {
    let stats = use_state(Vec::new);
    {
        let stats = stats.clone();
        let summoner_id = *summoner_id;
        use_effect_with((), move |_| {
            let stats = stats.clone();
            let summoner_id = summoner_id;
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("https://api.påsan.no/summoner_stats/{summoner_id}");
                let mut fetches_stats: Vec<ChampionStats> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                fetches_stats.sort_by_key(|x| x.wins + x.losses);
                fetches_stats.reverse();
                stats.set(fetches_stats);
            });
            || ()
        });
    }

    let stats: Html = stats
        .iter()
        .map(|stat| {
            html! {<Stat stat={stat.clone()} />}
        })
        .collect();

    html! {
        <div id="sidebar">{stats}</div>
    }
}

#[function_component(MatchHistory)]
fn match_history(SummonerDetailsProps { summoner_id }: &SummonerDetailsProps) -> Html {
    let matches = use_state(Vec::new);
    {
        let matches = matches.clone();
        let summoner_id = *summoner_id;
        use_effect_with((), move |_| {
            let matches = matches.clone();
            let summoner_id = summoner_id;
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("https://api.påsan.no/summoner_matches/{summoner_id}");
                let mut fetched_matches: Vec<LolMatchHistoryMatchHistoryGame> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                fetched_matches.sort_by_key(|x| x.game_creation);
                fetched_matches.reverse();
                matches.set(fetched_matches);
            });
            || ()
        });
    }
    html! {
        <main id="main">
            <Stats summoner_id={summoner_id}/>
            <div id="matches"></div>
        </main>
    }
}

#[function_component(NavBar)]
fn nav_bar() -> Html {
    html! {
        <nav>
            <a href="/">{"Home"}</a>
            <a href="/match_history.html" class="current">{"Custom game stats"}</a>
            <a href="/clicker/index.htm">{"Kebab clicker (made by Ivar)"}</a>
        </nav>
    }
}

#[function_component(SummonerOptions)]
fn summoner_options(SummonerProps { summoners }: &SummonerProps) -> Html {
    summoners
        .iter()
        .map(|summoner| {
            html! {
                <option value={summoner.summoner_id.to_string()}>
                    {summoner.summoner_name.clone()}
                </option>
            }
        })
        .collect()
}

#[function_component(SummonerSelector)]
fn summoner_selector(SummonerSelectorProps { on_change }: &SummonerSelectorProps) -> Html {
    let on_summoner_select = {
        let on_change = on_change.clone();
        Callback::from(move |event: Event| {
            let select = event
                .target()
                .and_then(|t| t.dyn_into::<HtmlSelectElement>().ok())
                .and_then(|x| x.value().parse().ok());
            if let Some(select) = select {
                on_change.emit(select);
            }
        })
    };

    let summoners = use_state(Vec::new);
    {
        let summoners = summoners.clone();
        use_effect_with((), move |_| {
            let summoners = summoners.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let collator_l1 =
                    Collator::try_new(&DataLocale::from(locale!("no")), CollatorOptions::new())
                        .unwrap();
                let mut fetched_summoners: Vec<Summoner> =
                    Request::get("https://api.påsan.no/summoner_names")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                fetched_summoners
                    .sort_by(|x, y| collator_l1.compare(&x.summoner_name, &y.summoner_name));
                summoners.set(fetched_summoners);
            });
            || ()
        });
    }

    html! {
        <select onchange={on_summoner_select.clone()}>
            <option value=""></option>
            <SummonerOptions summoners={(*summoners).clone()} />
        </select>
    }
}

#[function_component(App)]
fn app() -> Html {
    let selected_summoner = use_state(|| None);

    let on_summoner_select = {
        let selected_summoner = selected_summoner.clone();
        Callback::from(move |summoner| selected_summoner.set(Some(summoner)))
    };

    let summoner = selected_summoner.as_ref().map(|summoner| {
        html! {
            <MatchHistory summoner_id={summoner}/>
        }
    });

    html! {
        <>
            <header>
                <NavBar />
                <h1 id="header">{"Custom Game Match History"}</h1>
                <SummonerSelector on_change={on_summoner_select} />
            </header>
            {for summoner}
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
