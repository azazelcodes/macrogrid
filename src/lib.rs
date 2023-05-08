use macroquad::prelude::*;

// TODO padding, write text

pub struct Grid {
    width: i32,  // width of the grid in pixels
    height: i32, // height of the grid in pixels

    width_cells: i32,                  // number of cells
    height_cells: i32,                 // number of cells
    bg_color: macroquad::color::Color, // color of the cells

    gap: i32, // space between cells (in pixels)
    gap_color: macroquad::color::Color,

    selected: Option<i32>, // selected cell (if needed)
    selected_color: Option<macroquad::color::Color>,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            width: screen_width() as i32,
            height: screen_height() as i32,
            width_cells: 10,
            height_cells: 10,
            bg_color: RED,
            gap: 3,
            gap_color: PINK,
            selected: None,
            selected_color: Some(BLUE),
        }
    }
}

impl Grid {
    pub fn new(width: i32, height: i32, x_cells: i32, y_cells: i32, gap: i32) -> Self {
        Grid {
            width,
            height,
            width_cells: x_cells,
            height_cells: y_cells,
            bg_color: RED,
            gap,
            gap_color: BLACK,
            selected: None,
            selected_color: Some(BLUE),
        }
    }

    // returns the (width, height) of each cell
    fn calculate_dimensions(&self) -> (i32, i32) {
        // in pixels
        let total_x_gap_space = (self.width_cells - 1) * self.gap;
        let total_y_gap_space = (self.height_cells - 1) * self.gap;

        let cell_width = (self.width - total_x_gap_space) / self.width_cells;
        let cell_height = (self.height - total_y_gap_space) / self.height_cells;

        (cell_width, cell_height)
    }

    pub fn draw(&self) {
        // draw background (the gap color)
        draw_rectangle(
            0.0,
            0.0,
            self.width as f32,
            self.height as f32,
            self.gap_color,
        );

        // draw cells
        let (cell_width, cell_height) = self.calculate_dimensions();

        for i in 0..self.height_cells {
            for j in 0..self.width_cells {
                let x_pos = j * (cell_width + self.gap);
                let y_pos = i * (cell_height + self.gap);

                draw_rectangle(
                    x_pos as f32,
                    y_pos as f32,
                    cell_width as f32,
                    cell_height as f32,
                    self.bg_color,
                );
            }
        }
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
