#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{
    theme::Theme,
    widget::{Button, Checkbox, Column, Container, Element, Row, Text},
};
use client_api::{
    actions::{
        create_custom, get_online_friends, invite_to_lobby, randomize_teams, DraftType, Map,
    },
    client::Client,
    Error,
};
use eyre::Result;
use iced::{executor, window::icon, Application, Command, Length, Settings};
use image::ImageFormat;
use std::{collections::BTreeMap, sync::Arc};

mod theme;
mod widget;

const SPACING: u16 = 22;

fn main() -> Result<()> {
    // $env:RUST_LOG = "lol_utilities"
    env_logger::init();
    App::run(Settings {
        window: iced::window::Settings {
            size: (600, 300),
            resizable: true,
            decorations: true,
            icon: Some(icon::from_file_data(
                include_bytes!(r"../NeekoSquare.png"),
                Some(ImageFormat::Png),
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
}

#[derive(Debug, Clone)]
enum Message {
    CreateTournamentDraftLobby,
    CreateBlindPickLobby,
    RandomizeTeams,
    Invite,
    AttemptConnection,
    Connect(Option<InnerApp>),
    Disconnect,
    FriendToggled(Summoner),
    UpdateFriends,
    UpdatedFriends(BTreeMap<Summoner, bool>),
    Nothing,
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            App { inner: None },
            Command::perform(create_inner_app(), |inner| Message::Connect(inner.ok())),
        )
    }

    fn title(&self) -> String {
        String::from("League of Legends Utilities")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::CreateTournamentDraftLobby => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move {
                        create_custom(&client, DraftType::TorunamentDraft, Map::SummonersRift).await
                    },
                    check_api_response("Created lobby", "Failed to create lobby"),
                )
            }
            Message::CreateBlindPickLobby => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { create_custom(&client, DraftType::BlindPick, Map::HowlingAbyss).await },
                    check_api_response("Created lobby", "Failed to create lobby"),
                )
            }
            Message::RandomizeTeams => {
                let client = self.inner.as_ref().unwrap().api_client.clone();
                Command::perform(
                    async move { randomize_teams(&client).await },
                    check_api_response("Randomized teams", "Failed to randomize teams"),
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
                    check_api_response("Invited friends", "Failed to invite friends"),
                )
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
            Message::Connect(Some(inner)) => {
                self.inner = Some(inner);
                log::info!("Connected to client");
                Command::none()
            }
            Message::Connect(None) => {
                log::error!("Failed to connect to client");
                Command::none()
            }
            Message::AttemptConnection => {
                log::info!("Attempting to connect to client");
                Command::perform(create_inner_app(), |inner| Message::Connect(inner.ok()))
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
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        #[allow(clippy::single_match_else)]
        let content: Element<'_, _> = match self.inner.as_ref() {
            Some(inner) => {
                let create_lobby_button =
                    Button::new("Create lobby!").on_press(Message::CreateTournamentDraftLobby);

                let create_aram_lobby_button =
                    Button::new("Create ARAM lobby!").on_press(Message::CreateBlindPickLobby);

                let create_lobby_column = Column::with_children(vec![
                    create_lobby_button.into(),
                    create_aram_lobby_button.into(),
                ])
                .spacing(6);

                let update_friends_list_button =
                    Button::new("Update friends list").on_press(Message::UpdateFriends);

                let checkmarks_column = inner
                    .friends
                    .iter()
                    .fold(Column::new(), |column, (friend, checked)| {
                        column.push(Checkbox::new(friend.name.clone(), *checked, |_| {
                            Message::FriendToggled(friend.clone())
                        }))
                    })
                    .spacing(6);

                let friends_list_column = Column::with_children(vec![
                    update_friends_list_button.into(),
                    checkmarks_column.into(),
                ])
                .spacing(SPACING);

                let invite_button = Button::new("Invite!").on_press(Message::Invite);

                let randomize_teams_button =
                    Button::new("Randomize teams!").on_press(Message::RandomizeTeams);

                Row::with_children(vec![
                    create_lobby_column.into(),
                    friends_list_column.into(),
                    invite_button.into(),
                    randomize_teams_button.into(),
                ])
                .spacing(SPACING)
                .into()
            }
            None => {
                let client_not_found_text = Text::new("Client not found");

                let connect_to_client_button =
                    Button::new("Connect to client").on_press(Message::AttemptConnection);

                Column::with_children(vec![
                    client_not_found_text.into(),
                    connect_to_client_button.into(),
                ])
                .spacing(SPACING)
                .into()
            }
        };

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
) -> impl FnOnce(Result<(), Error>) -> Message {
    move |resp| {
        if let Err(e) = resp {
            log::error!("{err_msg} ({e})");
            if matches!(e, Error::Request(_)) {
                Message::Disconnect
            } else {
                Message::Nothing
            }
        } else {
            log::info!("{ok_msg}");
            Message::Nothing
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

async fn create_inner_app() -> Result<InnerApp, Error> {
    let api_client = Arc::new(Client::new()?);
    let friends = get_friends(&api_client).await?;

    Ok(InnerApp {
        api_client,
        friends,
    })
}
