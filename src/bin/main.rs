use macroquad::prelude::*;

use macroquad_grid::grid::Grid;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut g = Grid::new(500.0, 500.0, 6, 6, 5.0);
    //let g = Grid::default()

    g.select(1, 1);

    g.cells[3][3].value = Some(String::from("hi"));
    loop {
        clear_background(GREEN);

        g.draw();

        next_frame().await
    }
}
