use enigo::*;
use iced::{
    alignment,
    widget::{button, column, container, scrollable, text, text_input, Column},
    window, Color, Element, Length, Sandbox, Settings,
};
use rdev::{listen, simulate, Button, Event, EventType};
use std::sync::Arc;
use std::{sync::Mutex, thread, time};

struct Position {
    x: i32,
    y: i32,
    clicking: Arc<Mutex<bool>>,
}

#[derive(Debug, Clone)]
pub enum Message {
    XInputChanged(String),
    YInputChanged(String),
    ButtonClick,
    // SetX, // InputChanged(String),
}

impl Sandbox for Position {
    type Message = Message;

    fn new() -> Self {
        Self {
            x: 500,
            y: 500,
            clicking: Arc::new(Mutex::new(false)),
        }
    }

    fn title(&self) -> String {
        String::from("鼠标连点")
    }

    fn view(&self) -> Element<Message> {
        let title = text("click")
            .width(Length::Fill)
            .size(100)
            .style(Color::from([0.5, 0.5, 0.5]))
            .horizontal_alignment(alignment::Horizontal::Center);

        let inputx = text_input("x坐标", &self.x.to_string(), Message::XInputChanged)
            // .id(INPUT_ID.clone())
            .padding(15)
            .size(30)
            .on_submit(Message::XInputChanged("99".to_string()));
        let inputy = text_input("y坐标", &self.y.to_string(), Message::XInputChanged)
            // .id(INPUT_ID.clone())
            .padding(15)
            .size(30);

        // let click_button = button("start").on_press(Message::ButtonClick);
        let content = column![
            title,
            inputx,
            inputy,
            button("start").on_press(Message::ButtonClick)
        ]
        .spacing(20)
        .max_width(800);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(40)
                .center_x(),
        )
        .into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::XInputChanged(num_string) => {
                self.x = num_string.parse::<i32>().unwrap();
                println!("{}", self.x);
            }
            Message::YInputChanged(num_string) => {
                self.y = num_string.parse::<i32>().unwrap();
                println!("{}", self.y);
            }
            Message::ButtonClick => {
                // let mut enigo = Enigo::new();
                // enigo.mouse_move_to(self.x, self.y);
                // enigo.mouse_click(MouseButton::Left);
                let mut state = self.clicking.lock().unwrap();
                if *state == true {
                    return;
                }
                *state = true;
                drop(state);

                let clicking_copy = self.clicking.clone();
                let t = thread::spawn(move|| {
                    // let clicking_copy = self.clicking.clone();
                    if let Err(error) = listen(move |eve| Self::callback(clicking_copy.clone(), eve)) {
                        println!("Error: {:?}", error)
                    };
                });

                while true {
                    if *self.clicking.lock().unwrap() == true {
                        // let ten_millis = time::Duration::from_millis(500);
                        // thread::sleep(ten_millis);
                        println!("true");
                        Self::send(&EventType::ButtonPress(Button::Left));
                        Self::send(&EventType::ButtonRelease(Button::Left));
                        // Self::send(&EventType::ButtonRelease(Button::Right));
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

impl Position {
    fn send(event_type: &EventType) {
        let delay = time::Duration::from_millis(20);
        match simulate(event_type) {
            Ok(()) => (),
            Err(SimulateError) => {
                println!("We could not send {:?}", event_type);
            }
        }
        // Let ths OS catchup (at least MacOS)
        thread::sleep(delay);
    }

    fn callback(clicking: Arc<Mutex<bool>>, event: Event) {
        println!("My callback {:?}", event);
        match event.name {
            Some(string) => println!("User wrote {:?}", string),
            None => match event.event_type {
                rdev::EventType::KeyPress(rdev::Key::Alt) => {
                    *clicking.lock().unwrap() = false;
                    panic!("end thread");
                }
                _ => {}
            },
        }
    }
}

// fn main() {
//     println!("Hello, world!");
//     let mut enigo = Enigo::new();
//     let ten_millis = time::Duration::from_millis(500);
//     enigo.mouse_move_to(500, 200);
//     enigo.mouse_click(MouseButton::Left);
//     thread::sleep(ten_millis);
//     // enigo.key_sequence_parse("{+CTRL}a{-CTRL}{+SHIFT}Hello World{-SHIFT}");
// }

// This will block.

pub fn main() -> iced::Result {
    Position::run(Settings {
        window: window::Settings {
            size: (500, 400),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
