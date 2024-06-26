#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{
    theme::Theme,
    widget::{Column, Container, Element},
};
use client_api::{
    actions::{
        create_custom, get_online_friends, invite_from_previous, invite_to_lobby,
        post_custom_games_to_pasanapi, randomize_teams, DraftType, Map,
    },
    client::Client,
    Error,
};
use eyre::Result;
use iced::{
    alignment::Horizontal,
    executor,
    widget::column,
    widget::{button, checkbox, container, row, scrollable, slider, text},
    window::icon,
    Application, Command, Length, Settings,
};
use image::ImageFormat;
use std::{collections::BTreeMap, sync::Arc};

mod theme;
mod widget;

const SPACING: u16 = 22;
const ELEMENT_WIDTH: u16 = 170;

fn main() -> Result<()> {
    // $env:RUST_LOG = "lol_utilities,client_api"
    env_logger::init();
    App::run(Settings {
        window: iced::window::Settings {
            size: (1000, 300),
            resizable: true,
            decorations: true,
            icon: Some(icon::from_file_data(
                include_bytes!(r"../NeekoSquare.ico"),
                Some(ImageFormat::Ico),
            )?),
            ..Default::default()
        },
        ..Default::default()
    })?;
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Summoner {
    name: String,
    id: u64,
}

struct App {
    inner: Option<InnerApp>,
}

#[derive(Debug, Clone)]
struct InnerApp {
    api_client: Arc<Client>,
    friends: BTreeMap<Summoner, bool>,
    sending_games: bool,
    num_matches_to_check: u8,
}

#[derive(Debug, Clone)]
enum Message {
    CreateTournamentDraftLobby,
    CreateBlindPickLobby,
    RandomizeTeams,
    Invite,
    SendMatchHistory,
    DoneSendingMatchHistory,
    MatchesToCheckChanged(u8),
    AttemptConnection,
    Connect(InnerApp),
    Disconnect,
    FriendToggled(Summoner),
    UpdateFriends,
    UpdatedFriends(BTreeMap<Summoner, bool>),
    InvitePrevious,
    Nothing,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self { inner: None },
            Command::perform(async { create_inner_app() }, check_client_connection),
        )
    }

    fn title(&self) -> String {
        String::from("League of Legends Utilities")
    }

    #[allow(clippy::too_many_lines)]
    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CreateTournamentDraftLobby => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move {
                        create_custom(&client, DraftType::TorunamentDraft, Map::SummonersRift).await
                    },
                    check_api_response("Created lobby", "Failed to create lobby", Message::Nothing),
                )
            }
            Message::CreateBlindPickLobby => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { create_custom(&client, DraftType::BlindPick, Map::HowlingAbyss).await },
                    check_api_response("Created lobby", "Failed to create lobby", Message::Nothing),
                )
            }
            Message::RandomizeTeams => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { randomize_teams(&client).await },
                    check_api_response(
                        "Randomized teams",
                        "Failed to randomize teams",
                        Message::Nothing,
                    ),
                )
            }
            Message::Invite => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                let friends = self
                    .inner
                    .as_ref()
                    .unwrap()
                    .friends
                    .iter()
                    .filter_map(|(summ, check)| if *check { Some(summ.id) } else { None })
                    .collect::<Vec<_>>();
                Command::perform(
                    async move { invite_to_lobby(&client, &friends).await },
                    check_api_response(
                        "Invited friends",
                        "Failed to invite friends",
                        Message::Nothing,
                    ),
                )
            }
            Message::SendMatchHistory => {
                let num_matches_to_check = self.inner.as_ref().unwrap().num_matches_to_check;
                self.inner.as_mut().unwrap().sending_games = true;
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { post_custom_games_to_pasanapi(&client, num_matches_to_check).await },
                    check_api_response(
                        "Sent custom games",
                        "Failed to send custom games",
                        Message::DoneSendingMatchHistory,
                    ),
                )
            }
            Message::DoneSendingMatchHistory => {
                self.inner.as_mut().unwrap().sending_games = false;
                Command::none()
            }
            Message::UpdateFriends => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { get_friends(&client).await },
                    move |resp| match resp {
                        Ok(friends) => {
                            log::info!("Updated friends list");
                            Message::UpdatedFriends(friends)
                        }
                        Err(e) => {
                            log::error!("Failed to update friends list: {e}");
                            if matches!(e, Error::Request(_)) {
                                Message::Disconnect
                            } else {
                                Message::Nothing
                            }
                        }
                    },
                )
            }
            Message::FriendToggled(summoner) => {
                if let Some(value) = self.inner.as_mut().unwrap().friends.get_mut(&summoner) {
                    *value = !*value;
                    log::info!(
                        r#"Toggled {} "{}""#,
                        if *value { "on" } else { "off" },
                        summoner.name
                    );
                } else {
                    log::error!(r#"Failed to find friend "{}""#, summoner.name);
                }
                Command::none()
            }
            Message::InvitePrevious => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { invite_from_previous(&client).await },
                    check_api_response(
                        "Invited previous",
                        "Failed to invite previous",
                        Message::Nothing,
                    ),
                )
            }
            Message::Connect(inner) => {
                self.inner = Some(inner);
                Command::perform(async {}, |()| Message::UpdateFriends)
            }
            Message::AttemptConnection => {
                Command::perform(async { create_inner_app() }, check_client_connection)
            }
            Message::Nothing => Command::none(),
            Message::Disconnect => {
                log::info!("Disconnecting from client");
                self.inner = None;
                Command::none()
            }
            Message::UpdatedFriends(friends) => {
                self.inner.as_mut().unwrap().friends = friends;
                Command::none()
            }
            Message::MatchesToCheckChanged(i) => {
                self.inner.as_mut().unwrap().num_matches_to_check = i;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        #[allow(clippy::single_match_else)]
        let content: Element<'_, _> = self.inner.as_ref().map_or_else(
            || {
                column!(
                    text("Client not found"),
                    button("Connect to client").on_press(Message::AttemptConnection),
                )
                .spacing(SPACING)
                .into()
            },
            |inner| {
                let create_lobby_column = column!(
                    button("Create lobby!")
                        .on_press(Message::CreateTournamentDraftLobby)
                        .width(ELEMENT_WIDTH),
                    button("Create ARAM lobby!")
                        .on_press(Message::CreateBlindPickLobby)
                        .width(ELEMENT_WIDTH),
                )
                .spacing(6);

                let checkmarks_column = container(
                    inner
                        .friends
                        .iter()
                        .fold(Column::new(), |column, (friend, checked)| {
                            column.push(checkbox(friend.name.clone(), *checked, |_| {
                                Message::FriendToggled(friend.clone())
                            }))
                        })
                        .spacing(6),
                )
                .width(ELEMENT_WIDTH)
                .style(theme::Container::Bordered);

                let friends_list_column = column!(
                    button("Update friends list")
                        .on_press(Message::UpdateFriends)
                        .width(ELEMENT_WIDTH),
                    scrollable(checkmarks_column)
                        .height(200)
                        .width(ELEMENT_WIDTH)
                )
                .spacing(SPACING);

                let invite_column = column!(
                    button("Invite!")
                        .on_press(Message::Invite)
                        .width(ELEMENT_WIDTH),
                    button("Invite from previous game!")
                        .on_press(Message::InvitePrevious)
                        .width(ELEMENT_WIDTH)
                )
                .spacing(6);

                let send_match_history_column = column!(
                    if inner.sending_games {
                        button("Sending...").on_press_maybe(None)
                    } else {
                        button("Send match history!").on_press(Message::SendMatchHistory)
                    }
                    .width(ELEMENT_WIDTH),
                    slider(
                        1..=200,
                        inner.num_matches_to_check,
                        Message::MatchesToCheckChanged
                    )
                    .width(ELEMENT_WIDTH),
                    text(format!(
                        "Number of games to check: {}",
                        inner.num_matches_to_check
                    ))
                    .horizontal_alignment(Horizontal::Center)
                    .width(ELEMENT_WIDTH)
                )
                .spacing(SPACING);

                row!(
                    create_lobby_column,
                    friends_list_column,
                    invite_column,
                    button("Randomize teams!")
                        .on_press(Message::RandomizeTeams)
                        .width(ELEMENT_WIDTH),
                    send_match_history_column,
                )
                .spacing(SPACING)
                .into()
            },
        );

        Container::new(content)
            .style(theme::Container::Bordered)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn check_api_response(
    ok_msg: &'static str,
    err_msg: &'static str,
    message_end: Message,
) -> impl FnOnce(Result<(), Error>) -> Message {
    move |resp| {
        if let Err(e) = resp {
            log::error!("{err_msg} ({e})");
            if matches!(e, Error::Request(_)) {
                Message::Disconnect
            } else {
                message_end
            }
        } else {
            log::info!("{ok_msg}");
            message_end
        }
    }
}

async fn get_friends(api_client: &Client) -> Result<BTreeMap<Summoner, bool>, Error> {
    Ok(get_online_friends(api_client)
        .await?
        .into_iter()
        .map(|x| {
            (
                Summoner {
                    name: x.name,
                    id: x.summoner_id,
                },
                true,
            )
        })
        .collect())
}

fn create_inner_app() -> Result<InnerApp, Error> {
    let api_client = Arc::new(Client::new()?);

    Ok(InnerApp {
        api_client,
        friends: BTreeMap::new(),
        sending_games: false,
        num_matches_to_check: 10,
    })
}

fn check_client_connection(res: Result<InnerApp, Error>) -> Message {
    log::info!("Attempting to connect to client");
    match res {
        Ok(client) => {
            log::info!("Connected to client");
            Message::Connect(client)
        }
        Err(e) => {
            log::error!("Failed to connect to client: {e}");
            Message::Nothing
        }
    }
}
