use fixed_vectors::Vector2;
use yew::prelude::*;


#[derive(Debug)]
pub struct App {
    /// Grid of cell positions
    grid: Vec<Vector2<u32>>,
}


#[derive(Debug, PartialEq)]
pub enum AppMessage {
    /// Event to place a cell at a specified position.
    PlaceCell { position: Vector2<u32> },

    /// Event to remove a cell at a specified position.
    RemoveCell { position: Vector2<u32> },
}


impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            grid: Vec::new(),
        };
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <div>
                <h1>{ "Hello, World!" }</h1>
            </div>
        };
    }
}