use macroquad::prelude::*;

use macroquad_grid::Grid;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(GREEN);

        //macroquad_grid::temp();
        let g = Grid::new(200, 200, 10, 10, 5);

        g.draw();

        next_frame().await
    }
}
