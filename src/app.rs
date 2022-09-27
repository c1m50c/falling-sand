use crate::logic::{Cell, CellChunk};

use std::collections::HashMap;
use fixed_vectors::Vector2;
use yew::prelude::*;


#[derive(Debug)]
pub struct App {
    chunks: HashMap<Vector2<u32>, CellChunk>,
}


#[derive(PartialEq)]
pub enum AppMessage {
    /// Event to place a cell at a specified position.
    PlaceCell { global_position: Vector2<u32>, local_position: Vector2<u32>, material: u32 },

    /// Event to remove a cell at a specified position.
    RemoveCell { global_position: Vector2<u32>, local_position: Vector2<u32> },

    Proccessing,

    Paused,
}


impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            chunks: HashMap::new(),
        };
    }

    #[allow(unreachable_patterns)]
    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        return match message {
            AppMessage::PlaceCell { global_position, local_position, material } => {
                let chunk = self.chunks.get_mut(&global_position)
                    .expect(format!("Expected a CellChunk at position `{:?}`.", &global_position).as_str());
                chunk[local_position].material = material;
                true
            },

            AppMessage::RemoveCell { global_position, local_position } => {
                let chunk = self.chunks.get_mut(&global_position)
                    .expect(format!("Expected a CellChunk at position `{:?}`.", &global_position).as_str());
                chunk[local_position].material = 0;
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