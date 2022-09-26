use crate::logic::{Cell, position_to_index};
use fixed_vectors::Vector2;
use yew::prelude::*;


pub const DEFAULT_DIMENSIONS: Vector2<u32> = Vector2::new(512, 512);


#[derive(Debug)]
pub struct App {
    /// Dimensions of the [`grid`].
    dimensions: Vector2<u32>,

    /// Contains all [`Cell`]s we use to simulate.
    grid: Vec<Cell>,
}


#[derive(PartialEq)]
pub enum AppMessage {
    /// Event to place a cell at a specified position.
    PlaceCell { position: Vector2<u32>, material: u32 },

    /// Event to remove a cell at a specified position.
    RemoveCell { position: Vector2<u32> },
}


impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            dimensions: DEFAULT_DIMENSIONS,
            grid: Vec::with_capacity(
                (DEFAULT_DIMENSIONS.x * DEFAULT_DIMENSIONS.y) as usize
            ),
        };
    }

    #[allow(unreachable_patterns)]
    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        return match message {
            AppMessage::PlaceCell { position, material } => {
                let index = position_to_index(&position, &self.dimensions);
                self.grid[index] = Cell::new(material);
                true
            },

            AppMessage::RemoveCell { position } => {
                let index = position_to_index(&position, &self.dimensions);
                self.grid[index] = Cell::new(0);
                true
            },

            _ => false,
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