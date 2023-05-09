use macroquad::prelude::*;

// TODO padding, write text

pub struct Grid {
    width: f32,  // width of the grid in pixels
    height: f32, // height of the grid in pixels

    width_cells: i32,                  // number of cells
    height_cells: i32,                 // number of cells
    bg_color: macroquad::color::Color, // color of the cells

    gap: f32, // space between cells (in pixels)
    gap_color: macroquad::color::Color,

    selected_cell: Option<(i32, i32)>, // selected cell (if needed)
    selected_color: Option<macroquad::color::Color>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: screen_width(),
            height: screen_height(),
            width_cells: 10,
            height_cells: 10,
            bg_color: RED,
            gap: 3.0,
            gap_color: PINK,
            selected_cell: None,
            selected_color: Some(BLUE),
        }
    }
}

impl Grid {
    pub fn new(width: f32, height: f32, x_cells: i32, y_cells: i32, gap: f32) -> Self {
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
                let x_pos = self.gap + j as f32 * (cell_width + self.gap as f32);
                let y_pos = self.gap + i as f32 * (cell_height + self.gap as f32);

                let mut color = self.bg_color;
                // if this is the selected_cell, use the other color
                if let Some((row, col)) = self.selected_cell {
                    if row == i && col == j {
                        color = self
                            .selected_color
                            .expect("there was a selected cell but no selected color");
                    }
                }

                draw_rectangle(x_pos, y_pos, cell_width, cell_height, color);
            }
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

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {
        todo!()
    }
}
