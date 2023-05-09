use macroquad::prelude::*;

use macroquad_grid::grid::Grid;

#[macroquad::main("BasicShapes")]
async fn main() {
    //macroquad_grid::temp();
    let mut g = Grid::new(500.0, 500.0, 6, 6, 5.0);
    //let g = Grid::default()

    loop {
        clear_background(GREEN);

        g.draw();

        g.select(1, 1);

        next_frame().await
    }
}
