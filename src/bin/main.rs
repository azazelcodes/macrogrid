use macroquad::prelude::*;

use macroquad_grid::Grid;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(BLUE);

        //macroquad_grid::temp();
        let g = Grid::default();

        g.draw();

        next_frame().await
    }
}
