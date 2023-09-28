use client_api::actions::{get_online_friends, invite_to_lobby, randomize_teams};
use client_api::{actions::create_custom, client::Client};
use eyre::Result;
use iced::widget::{button, checkbox, container, row, Column};
use iced::{Element, Length, Sandbox, Settings};
use std::collections::HashMap;

fn main() -> Result<()> {
    App::run(Settings::default())?;
    Ok(())
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Summoner {
    name: String,
    id: u64,
}

struct App {
    api_client: Client,
    friends: HashMap<Summoner, bool>,
}

#[derive(Debug, Clone)]
enum Message {
    CreateLobby,
    RandomizeTeams,
    Invite,
    Checked(Summoner),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        let api_client = Client::new().expect("Failed to create client");
        let friends: HashMap<_, _> = get_online_friends(&api_client)
            .expect("Failed to find friends")
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
            .collect();
        App {
            api_client,
            friends,
        }
    }

    fn title(&self) -> String {
        String::from("League of Legends Utilities")
    }

    fn update(&mut self, message: Self::Message) {
        let res = match message {
            Message::CreateLobby => create_custom(&self.api_client),
            Message::RandomizeTeams => randomize_teams(&self.api_client),
            Message::Invite => invite_to_lobby(
                &self.api_client,
                &self
                    .friends
                    .iter()
                    .filter(|(_, &x)| x)
                    .map(|(x, _)| x.id)
                    .collect::<Vec<_>>(),
            ),
            Message::Checked(name) => {
                let value = self.friends.get_mut(&name).unwrap();
                *value = !*value;
                Ok(())
            }
        };
        if let Err(e) = res {
            println!("{e}");
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let checkmarks_column =
            self.friends
                .iter()
                .fold(Column::new(), |column, (friend, checked)| {
                    column.push(checkbox(friend.name.clone(), *checked, |_| {
                        Message::Checked(friend.clone())
                    }))
                });
        let content = row![
            button("Create lobby!").on_press(Message::CreateLobby),
            checkmarks_column,
            button("Invite!").on_press(Message::Invite),
            button("Randomize teams!").on_press(Message::RandomizeTeams)
        ]
        .spacing(22);

        container(content)
            .width(Length::Fill)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
