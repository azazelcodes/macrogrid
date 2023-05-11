use macroquad::prelude::*;

use macroquad_grid::grid::Grid;

// shows off
// - drawing the grid
// - selecting a cell
// - changing selected cells color
// - changing default cell bg color
// - changing gap color
// - changing grids postion with Position enum
// - setting color of a specific cell
// - writing text to a cell
// - 
// -
// -

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut g = Grid::new(500.0, 500.0, 6, 6, 5.0);
    //let g = Grid::default()

    g.select_cell(1, 1);

    g.color_cell(2, 2, YELLOW);

    g.set_cell_text(3, 3, Some("hi"));
    loop {
        clear_background(GREEN);

        g.draw();

        next_frame().await
    }
}
