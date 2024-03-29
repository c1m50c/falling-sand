use crate::logic::cell::Cell;

use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use std::collections::HashMap;
use fixed_vectors::Vector2;
use wasm_bindgen::JsCast;
use yew::prelude::*;


#[derive(Debug)]
pub struct App {
    pub cells: HashMap<Vector2<u32>, Cell>,
    canvas: NodeRef,
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
        let mut map = HashMap::new();

        for x in 0..128 {
            for y in 0..128 {
                if x % 2 == 0 && y % 2 == 0 {
                    map.insert(Vector2::new(x, y), Cell::new(0));
                }
            }
        }

        return Self {
            cells: map,
            canvas: NodeRef::default(),
        };
    }

    #[allow(unreachable_patterns)]
    fn update(&mut self, _ctx: &Context<Self>, message: Self::Message) -> bool {
        return match message {
            AppMessage::PlaceCell { position, material } => {
                self.cells.insert(position, Cell::new(material));
                true
            },

            AppMessage::RemoveCell { position } => {
                self.cells.remove(&position);
                true
            },

            AppMessage::PlaceCells { positions, material } => {
                positions.into_iter()
                    .for_each(|p| {
                        self.cells.insert(p, Cell::new(material));
                    });
                true
            },

            AppMessage::RemoveCells { positions } => {
                positions.iter()
                    .for_each(|p| {
                        self.cells.remove(&p);
                    });
                true
            }

            _ => false,
        };
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>()
            .expect("Expected a `HtmlCanvasElement` for `canvas`.");
        
        let ctx2d: CanvasRenderingContext2d = canvas.get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        
        crate::rendering::render(
            self, ctx, &ctx2d, 
            Vector2::new(1.0, 1.0)
        );
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
            <canvas
                ref={self.canvas.clone()}
                style="min-height: 100%; min-width: 100%; image-rendering: pixelated; image-rendering: crisp-edges;"
            />
        };
    }
}