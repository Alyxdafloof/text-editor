//EXAMPLE TEXT EDITOR USING ICED

//LIBRARIES USED
use iced::{Element, Sandbox, Settings};
use iced::widget::text;

//START OF PROGRAM
fn main () -> iced::Result {
    Editor::run(Settings::default())
}

struct Editor;

#[derive(Debug)]
enum Message {}

impl Sandbox for Editor {

//messages are user events that can change the state of the application
    type Message = Message;

//initalizes the state of the app
    fn new() -> Self {
        Self
    }

//its a title
    fn title(&self) -> String {
        String::from("text-editor-rs")
    }

//handles user events and is what changes the app state
    fn update(&mut self, message: Message) {
        match message {}
    }

//where the actuall elements of the display are handled
//element is generic
//text is a specific widgit which this program cant handle
//you add ".into()" so now it can return a generic widgit and work in this program
    fn view(&self) -> iced::Element<'_, Message> {
        text("Hello Iced").into()
    }
}
