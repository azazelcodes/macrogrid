use macroquad::prelude::*;

use macroquad_grid::Grid;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        //macroquad_grid::temp();
        let g = Grid::new(1, 1, 0);

        // let (cell_width, cell_height) = g.calculate_dimensions();

        next_frame().await
    }
}
