pub use crate::Grid;
use macroquad::prelude::*;

// only called from the double for loop in the draw function
// this way it does not look crouded as fuck
//
// this function calculates the cells position (takes gap into account)
// it also handles any special coloring that might need to happen
// it also prints any text to the screen (if applicable)
pub fn draw_cell(
    grid: &Grid,
    row: usize,
    col: usize,
    cell_width: f32,
    cell_height: f32,
    x_offset: f32,
    y_offset: f32,
) {
    // cell cords
    let x_pos = x_offset + grid.gap + col as f32 * (cell_width + grid.gap as f32);
    let y_pos = y_offset + grid.gap + row as f32 * (cell_height + grid.gap as f32);

    // cell color
    let mut color = grid.cell_bg_color;
    // if this is the selected_cell, use the other color
    if let Some((selected_row, selected_col)) = grid.selected_cell {
        if selected_row == row && selected_col == col {
            color = grid
                .selected_color
                .expect("there was a selected cell but no selected color");
        }
    }
    // and if it had a preset color then use that
    else if let Some(set_color) = grid.cells[row][col].color {
        // somehow we never reach this??
        color = set_color;
    }

    // draw it!
    draw_rectangle(x_pos, y_pos, cell_width, cell_height, color);

    // draw the text if this cell has any
    if let Some(text) = &grid.cells[row][col].text {
        // shifted because read the readme
        let y_pos = y_pos + cell_height;

        // center the text or something idk
        let text_dim = macroquad::text::measure_text(text, None, cell_height as u16, 1.0); // 1.0 is default
        let centered_x = (cell_width - text_dim.width) / 2.0 + x_pos;
        let centered_y = y_pos - (cell_height - text_dim.height) / 2.0;

        draw_text(text, centered_x, centered_y, cell_height, BLACK);
    }
}
