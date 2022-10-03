use crate::app::App;

use web_sys::CanvasRenderingContext2d;
use fixed_vectors::Vector2;
use yew::Context;


pub fn render(app: &mut App, _context: &Context<App>, ctx2d: &CanvasRenderingContext2d, dimensions: Vector2<f64>) {    
    let _ = &app.cells.iter().for_each(|(position, cell)| {
        draw_cell(ctx2d, &dimensions, position, &cell.material);
    });
}


fn draw_cell(ctx2d: &CanvasRenderingContext2d, dimensions: &Vector2<f64>, position: &Vector2<u32>, _material: &u32) {
    let float = Vector2::new(position.x as f64, position.y as f64) * dimensions.clone();    
    ctx2d.fill_rect(float.x, float.y, dimensions.x, dimensions.y);
}