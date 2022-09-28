use crate::app::App;

use web_sys::WebGlRenderingContext;
use yew::Context;


#[derive(Debug)]
pub struct Renderer<'a> {
    app: &'a mut App,
    context: &'a Context<App>,
    web_gl: WebGlRenderingContext
}


impl<'a> Renderer<'a> {
    /// Constructs a new [`Renderer`] with its required fields.
    pub fn new(app: &'a mut App, context: &'a Context<App>, web_gl: WebGlRenderingContext) -> Self {
        return Self {
            app,
            context,
            web_gl,
        };
    }

    /// Primary function of the [`Renderer`],
    /// takes in the current state of the [`App`] and renders a scene to the specified [`WebGlRenderingContext`].
    pub fn render(self) {
        self.web_gl.clear_color(0.12, 0.12, 0.12, 1.0);
        self.web_gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    }
}