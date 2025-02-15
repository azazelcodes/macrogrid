pub use crate::Grid;
use macroquad::prelude::*;

// only called from the double for loop in the draw function
// this way it does not look crouded as fuck
//
// this function calculates the cells position
// this time the borders are acting as the gaps
// it also handles any special coloring that might need to happen
// it also prints any text to the screen (if applicable)
pub fn draw_cell(
    grid: &Grid,
    row: usize,
    col: usize,
    cell_size: f32,
    x_offset: f32,
    y_offset: f32,
    pointy: bool,
) {
    let (cell_width, cell_height) = if pointy {
        (3.0_f32.sqrt() * cell_size, 2.0 * cell_size)
    } else {
        (2.0 * cell_size, 3.0_f32.sqrt() * cell_size)
    };

    // cell cords THIS IS VERY MESSY IMSO SORRY!!
    // width vs height scaling
    let (x_scale, y_scale) = if pointy {
        (1.0, 3.0 / 4.0)
    } else {
        (3.0 / 4.0, 1.0)
    };    

    // staggerinh
    let (x_adjust, y_adjust) = if pointy {
        (if row % 2 == 0 { 0.5 * cell_width } else { 0.0 }, 0.0)
    } else {
        (0.0, if col % 2 == 0 { 0.5 * cell_height } else { 0.0 })
    };    

    let (x_pos, y_pos) = (
        x_offset + grid.gap + col as f32 * (x_scale * cell_width) - x_adjust,
        y_offset + grid.gap + row as f32 * (y_scale * cell_height) - y_adjust,
    );    
    // YAY CELL CORDS DONE!!
    /* previous one liner approach. can be removed but its beautiful in its own way
    let x_pos = x_offset + grid.gap + col as f32 * (if !pointy { 3.0/4.0 } else { 1.0 } * cell_width + grid.gap as f32) - if pointy { if row % 2 == 0 { 0.5 * cell_width } else { 0.0 } } else { 0.0 };
    let y_pos = y_offset + grid.gap + row as f32 * (if pointy { 3.0/4.0 } else { 1.0 } * cell_height + grid.gap as f32) - if pointy { 0.0 } else { if col % 2 == 0 { 0.5 * cell_height } else { 0.0 } };
    */


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
    // *1.15 to avoid some missing gap issues. very weird :P
    // I believe this to be a macroquad issue!
    draw_hexagon(x_pos, y_pos, cell_size, grid.gap * 1.15, pointy, grid.gap_color,color);

    // draw the text if this cell has any
    if let Some(text) = &grid.cells[row][col].text {
        // shifted because read the readme
        let y_pos = y_pos + cell_height * 0.5;

        // center the text or something idk
        let text_dim = macroquad::text::measure_text(text, None, (cell_height as f32 * 0.5) as u16, 1.0); // 1.0 is default
        let centered_x = (cell_width * 0.1 - text_dim.width) / 2.0 + x_pos;
        let centered_y = y_pos - (cell_height - text_dim.height) / 2.0;

        draw_text(text, centered_x, centered_y, cell_height as f32 * 0.5, WHITE);
    }
}
