#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use crate::{
    theme::Theme,
    widget::{Button, Checkbox, Column, Container, Element, Row, Text},
};
use client_api::{
    actions::{create_custom, get_online_friends, invite_to_lobby, randomize_teams},
    client::Client,
    Error,
};
use eyre::Result;
use iced::{executor, window::icon, Application, Command, Length, Settings};
use image::ImageFormat;
use std::collections::BTreeMap;

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
    api_client: Client,
    friends: BTreeMap<Summoner, bool>,
}

#[derive(Debug, Clone)]
enum Message {
    CreateLobby,
    RandomizeTeams,
    Invite,
    FriendToggled(Summoner),
    Connect(Option<InnerApp>),
    AttemptConnection,
    UpdateFriends,
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
            Message::CreateLobby => {
                if let Err(e) = create_custom(&self.inner.as_ref().unwrap().api_client) {
                    log::error!("Failed to create custom game lobby: {e}");
                    if matches!(e, Error::Request(_)) {
                        self.inner = None;
                        log::info!("Disconnecting from client");
                    }
                } else {
                    log::info!("Created lobby");
                }
                Command::none()
            }
            Message::RandomizeTeams => {
                if let Err(e) = randomize_teams(&self.inner.as_ref().unwrap().api_client) {
                    log::error!("Failed to randomize teams: {e}");
                    if matches!(e, Error::Request(_)) {
                        self.inner = None;
                        log::info!("Disconnecting from client");
                    }
                } else {
                    log::info!("Randomized teams");
                }
                Command::none()
            }
            Message::Invite => {
                let inner = self.inner.as_ref().unwrap();
                if let Err(e) = invite_to_lobby(
                    &inner.api_client,
                    &inner
                        .friends
                        .iter()
                        .filter(|(_, &x)| x)
                        .map(|(x, _)| x.id)
                        .collect::<Vec<_>>(),
                ) {
                    log::error!("Failed to invite players to lobby: {e}");
                    if matches!(e, Error::Request(_)) {
                        self.inner = None;
                        log::info!("Disconnecting from client");
                    }
                } else {
                    log::info!("Invited players");
                }
                Command::none()
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
            Message::UpdateFriends => {
                let inner = self.inner.as_mut().unwrap();
                match get_friends(&inner.api_client) {
                    Ok(friends) => {
                        inner.friends = friends;
                        log::info!("Updated friends list");
                    }
                    Err(e) => {
                        log::error!("Failed to update friends list: {e}");
                        if matches!(e, Error::Request(_)) {
                            self.inner = None;
                            log::info!("Disconnecting from client");
                        }
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        #[allow(clippy::single_match_else)]
        let content: Element<'_, _> = match self.inner.as_ref() {
            Some(inner) => {
                let create_lobby_button =
                    Button::new("Create lobby!").on_press(Message::CreateLobby);

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
                    checkmarks_column.into(),
                    update_friends_list_button.into(),
                ])
                .spacing(SPACING);

                let invite_button = Button::new("Invite!").on_press(Message::Invite);

                let randomize_teams_button =
                    Button::new("Randomize teams!").on_press(Message::RandomizeTeams);

                Row::with_children(vec![
                    create_lobby_button.into(),
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
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            // .center_y()
            .into()
    }
}

fn get_friends(api_client: &Client) -> Result<BTreeMap<Summoner, bool>, Error> {
    Ok(get_online_friends(api_client)?
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

#[allow(clippy::unused_async)]
async fn create_inner_app() -> Result<InnerApp, Error> {
    let api_client = Client::new()?;
    let friends = get_friends(&api_client)?;

    Ok(InnerApp {
        api_client,
        friends,
    })
}
