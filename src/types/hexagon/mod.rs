pub use crate::Grid;
use macroquad::prelude::*;

use crate::GridType;

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
) {
    let (cell_width, cell_height) = if grid.get_type() == GridType::HEXAGONP {
        (3.0_f32.sqrt() * cell_size, 2.0 * cell_size)
    } else {
        (2.0 * cell_size, 3.0_f32.sqrt() * cell_size)
    };

    // cell cords THIS IS VERY MESSY IMSO SORRY!!
    // width vs height scaling
    let (x_scale, y_scale) = if grid.get_type() == GridType::HEXAGONP {
        (1.0, 3.0 / 4.0)
    } else {
        (3.0 / 4.0, 1.0)
    };    

    // staggerinh
    let (x_adjust, y_adjust) = if grid.get_type() == GridType::HEXAGONP {
        (if row % 2 == 0 { 0.5 * cell_width } else { 0.0 }, 0.0) // ODD-R
    } else {
        (0.0, if col % 2 == 0 { 0.5 * cell_height } else { 0.0 }) // ODD-Q
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
    draw_hexagon(x_pos, y_pos, cell_size, grid.gap * 1.15, grid.get_type() == GridType::HEXAGONP, grid.gap_color,color);

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

/*
UNIVERSAL FUNCTIONS!! YIPPIE!

(i32, i32, i32) == cube coordinates
(i32, i32) == "normal" offset coordinates as used in draw_cell
*/
#[allow(dead_code)]
pub mod unifunc {
    pub fn to_cube(pointy: bool, row: i32, col: i32) -> (i32, i32, i32) {
        let (q, r) = if pointy {
            (col - (row + 1) / 2, row)
        } else {
            (col, row - (col + 1) / 2)
        };
        let s = -q-r;

        (q, r, s)
    }

    pub fn from_cube(pointy: bool, q: i32, r: i32) -> (i32, i32) {
        if pointy {
            (q + (r - (r + 1)) / 2, r)
        } else {
            (q, r + (q - (q + 1)) / 2)
        }
    }

    pub fn cube_dir(direction: usize) -> (i32, i32, i32) {
        let vectors: [(i32, i32, i32); 7] = [
            (0, 0, 0),                              // -> vectors[0] = center
            (1, 0, -1), (1, -1, 0), (0, -1, 1),
            (-1, 0, 1), (-1, 1, 0), (0, 1, -1),
        ];    
        /* 1 - 6   ^

        1   2
        /. .\ 
        6 . * . 3
        \. ./ 
        5   4
        
        */

        vectors[direction]
    }

    pub fn cube_add((cq, cr, cs): (i32, i32, i32), (vq, vr, vs): (i32, i32, i32)) -> (i32, i32, i32) {
        (cq + vq, cr + vr, cs + vs)
    }

    pub fn cube_sub((aq, ar, ass): (i32, i32, i32), (bq, br, bs): (i32, i32, i32)) -> (i32, i32, i32) {
        (aq + bq, ar + br, ass + bs) // ass =/= as
    }

    pub fn cube_mult((q, r, s): (i32, i32, i32), factor: i32) -> (i32, i32, i32) {
        (q * factor, r * factor, s * factor)
    }

    pub fn cube_dist(a: (i32, i32, i32), b: (i32, i32, i32)) -> i32 {
        let vector = cube_sub(a, b);
        (vector.0.abs() + vector.1.abs() + vector.2.abs()) / 2
    }

    pub fn cube_neig(cube: (i32, i32, i32), direction: usize) -> (i32, i32, i32) {
        cube_add(cube, cube_dir(direction))
    }

    pub fn cube_diag(cube: (i32, i32, i32), direction: usize) -> (i32, i32, i32) {
        let vectors: [(i32, i32, i32); 7] = [
            (0, 0, 0),                              // -> vectors[0] = center
            (1, -2, 1), (2, -1, -1), (1, 1, -2),
            (-1, 2, -1), (-2, 1, 1), (-1, -1, 2)
        ];
        /* 1 - 6   ^

            1
        6 /. .\ 2
        . * .
        5 \. ./ 3
            4
        
        */    
        return cube_add(cube, vectors[direction])
    }

    pub fn cube_ring(center: (i32, i32, i32), radius: i32) -> Vec<(i32, i32, i32)> { // NOT RADIUS 0!!
        let mut results: Vec<(i32, i32, i32)> = Vec::new();
        
        let mut hex = cube_add(center, cube_mult(cube_dir(5), radius));

        for i in 1..7 { // not 0..6 because 0 would be centers
            for _ in 0..radius {
                results.push(hex);
                hex = cube_neig(hex, i);
            }
        }    

        results
    }

    pub fn lerp_float(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    pub fn lerp_cube((aq, ar, ass): (i32, i32, i32), (bq, br, bs): (i32, i32, i32), t: f32) -> (i32, i32, i32) {
        (
            lerp_float(aq as f32, bq as f32, t) as i32,
            lerp_float(ar as f32, br as f32, t) as i32,
            lerp_float(ass as f32, bs as f32, t) as i32,
        )
    }

    pub fn cube_line(a: (i32, i32, i32), b: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
        let n = cube_dist(a, b);
        let mut results: Vec<(i32, i32, i32)> = Vec::new();
        for i in 0..n {
            results.push(lerp_cube(a, b, 1.0 / n as f32 * i as f32));
        }
        results
    }
}