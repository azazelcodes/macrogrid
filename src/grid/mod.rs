use macroquad::prelude::*;

mod cell;

pub struct Grid {
    width: f32,  // width of the grid in pixels
    height: f32, // height of the grid in pixels

    width_cells: i32,                  // number of cells
    height_cells: i32,                 // number of cells
    bg_color: macroquad::color::Color, // color of the cells

    gap: f32, // space between cells (in pixels)
    gap_color: macroquad::color::Color,

    // is a vec really needed here? how use const bro
    pub cells: Vec<Vec<cell::Cell>>,

    selected_cell: Option<(i32, i32)>, // selected cell (if needed)
    selected_color: Option<macroquad::color::Color>,
}

impl Default for Grid {
    fn default() -> Self {
        const WIDTH: i32 = 10;
        const HEIGHT: i32 = 10;
        Grid {
            width: screen_width(),
            height: screen_height(),
            width_cells: WIDTH,
            height_cells: HEIGHT,
            bg_color: RED,
            gap: 3.0,
            gap_color: PINK,
            selected_cell: None,
            selected_color: Some(BLUE),
            // ignore the HORRID line below this comment
            // it just makes a 2D list of cell::default's
            // there are HEIGHT inner lists and they all have WIDTH elements
            cells: (0..HEIGHT).into_iter().map(|_| (0..WIDTH).into_iter().map(|_| cell::Cell::default()).collect::<Vec<_>>()).collect()
        }
    }
}

impl Grid {
    pub fn new(width: f32, height: f32, x_cells: i32, y_cells: i32, gap: f32) -> Self {
        const WIDTH: i32 = 10;
        const HEIGHT: i32 = 10;
        Grid {
            width,
            height,
            width_cells: x_cells,
            height_cells: y_cells,
            bg_color: RED,
            gap,
            gap_color: BLACK,
            selected_cell: None,
            selected_color: Some(BLUE),
            // ignore the HORRID line below this comment
            // it just makes a 2D list of cell::default's
            // there are HEIGHT inner lists and they all have WIDTH elements
            cells: (0..HEIGHT).into_iter().map(|_| (0..WIDTH).into_iter().map(|_| cell::Cell::default()).collect::<Vec<_>>()).collect()
        }
    }

    // returns the (width, height) of each cell
    fn calculate_dimensions(&self) -> (f32, f32) {
        // in pixels
        let total_x_gap_space = (self.width_cells + 1) as f32 * self.gap;
        let total_y_gap_space = (self.height_cells + 1) as f32 * self.gap;

        let cell_width = (self.width - total_x_gap_space as f32) / self.width_cells as f32;
        let cell_height = (self.height - total_y_gap_space as f32) / self.height_cells as f32;

        (cell_width, cell_height)
    }

    pub fn draw(&self) {
        // draw background (the gap color)
        draw_rectangle(0.0, 0.0, self.width, self.height, self.gap_color);

        // draw cells
        let (cell_width, cell_height) = self.calculate_dimensions();

        for i in 0..self.height_cells {
            for j in 0..self.width_cells {
                self.draw_cell(i, j, cell_width, cell_height);
            }
        }
    }

    // only called from the double for loop in the draw function
    // this way it does not look crouded as fuck
    //
    // this function calculates the cells position (takes gap into account)
    // it also handles any special coloring that might need to happen
    // it also prints any text to the screen (if applicable)
    fn draw_cell(&self, row: i32, col: i32, cell_width: f32, cell_height: f32) {
        // cell cords
        let x_pos = self.gap + col as f32 * (cell_width + self.gap as f32);
        let y_pos = self.gap + row as f32 * (cell_height + self.gap as f32);

        // cell color
        let mut color = self.bg_color;
        // if this is the selected_cell, use the other color
        if let Some((selected_row, selected_col)) = self.selected_cell {
            if selected_row == row && selected_col == col {
                color = self
                    .selected_color
                    .expect("there was a selected cell but no selected color");
            }
        }

        // draw it!
        draw_rectangle(x_pos, y_pos, cell_width, cell_height, color);

        // draw the text if this cell has any
        if let Some(text) = &self.cells[row as usize][col as usize].value {
            draw_text(text, x_pos, y_pos + cell_height, cell_height, BLACK);
        }
    }

    pub fn select(&mut self, row: i32, col: i32) {
        self.selected_cell = Some((row, col))
    }

    pub fn color_cell(
        &mut self,
        row: i32,
        col: i32,
        color: macroquad::color::Color,
    ) -> anyhow::Result<()> {
        todo!()
    }
}
