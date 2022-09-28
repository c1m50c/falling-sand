use crate::logic::Cell;

use std::collections::HashMap;
use fixed_vectors::Vector2;
use yew::prelude::*;


#[derive(Debug)]
pub struct App {
    cells: HashMap<Vector2<u32>, Cell>,
}


#[derive(PartialEq)]
pub enum AppMessage {
    /// Places a [`Cell`] at the specified `position` in `cells` with the `material`.
    PlaceCell { position: Vector2<u32>, material: u32 },

    /// Removes a [`Cell`] at the specified `position` in `cells`.
    RemoveCell { position: Vector2<u32> },

    /// Places [`Cell`]s at the given `positions` with the `material`.
    PlaceCells { positions: Vec<Vector2<u32>>, material: u32 },

    /// Removes the [`Cell`]s at the given `positions`.
    RemoveCells { positions: Vec<Vector2<u32>> },
}


impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        return Self {
            cells: HashMap::new(),
        };
    }

    #[allow(unreachable_patterns)]
    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        return match message {
            AppMessage::PlaceCell { position, material } => {
                if let Some(cell) = self.cells.get_mut(&position) {
                    cell.material = material;
                }
                true
            },

            AppMessage::RemoveCell { position } => {
                self.cells.remove(&position);
                true
            },

            AppMessage::PlaceCells { positions, material } => {
                positions.iter()
                    .for_each(|p| {
                        if let Some(cell) = self.cells.get_mut(&p) {
                            cell.material = material;
                        }
                    });
                true
            },

            AppMessage::RemoveCells { positions } => {
                positions.iter()
                    .for_each(|p| { self.cells.remove(&p); });
                true
            }

            _ => false,
        };
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <canvas />
        };
    }
}