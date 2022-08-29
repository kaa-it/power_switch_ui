use iced::pure::{Element, Sandbox};
use iced::pure::widget::{Button, Text, TextInput, Toggler, Row, Column};
use iced::{Color, window};
use iced::Settings;

fn main() -> iced::Result {
    Application::run(Settings {
        window: window::Settings {
            size: (400, 300),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct Application {
    address: String,
    port: String,
    connected: bool,
    error: String,
    enabled: bool,
    power: String,
}

#[derive(Debug, Clone)]
enum Message {
    Connect,
    HostChanged(String),
    PortChanged(String),
    TurnPowerSwitch(bool)
}

impl Sandbox for Application {
    type Message = Message;

    fn new() -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: String::from("53453"),
            connected: false,
            error: String::new(),
            enabled: false,
            power: String::new(),
        }
    }

    fn title(&self) -> String {
        "Power Switch Client".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::HostChanged(value) => {
                self.address = value;
            }
            Message::PortChanged(value) => {
                self.port = value;
            }
            Message::Connect => {
                if self.connected {
                    self.connected = false;
                } else {
                    if !self.address.is_empty() && !self.port.is_empty() {
                        self.connected = true;
                        self.error = String::new();
                    } else {
                        if self.address.is_empty() {
                            self.error = String::from("Address is empty");
                        } else {
                            self.error = String::from("Port is empty");
                        }
                    }
                }
            }
            Message::TurnPowerSwitch(value) => {
                self.enabled = value;
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let host = TextInput::new("Host", &self.address, Message::HostChanged).padding(10);
        let port = TextInput::new("Port", &self.port, Message::PortChanged).padding(10);

        let button_title = if self.connected {
            "Disconnect"
        } else {
            "Connect"
        };
        let connect_button = Button::new(button_title).on_press(Message::Connect);

        let mut col = Column::new().padding(50).spacing(10).push(host).push(port).push(connect_button);

        if !self.error.is_empty() {
            let text = Text::new(self.error.clone()).color(Color::from_rgb(1.0, 0.0, 0.0));
            col = col.push(text);
        }

        if self.connected {
            let toggler = Toggler::new(self.enabled, "Enabled".to_string(), |b| Message::TurnPowerSwitch(b));
            let power = Text::new(format!("Power: {}", self.power)).size(20);

            col = col.push(toggler).push(power);
        }

        col.into()
    }
}
