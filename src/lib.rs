use macroquad::prelude::*;

// TODO padding, write text

pub struct Grid {
    width: i32, // number of cells
    height: i32, // number of cells
    gap: i32, // space between cells (in pixels)
    gap_color: macroquad::color::Color,
    selected: Option<i32>, // selected cell (if needed)
    selected_color: Option<macroquad::color::Color>
}

impl Default for Grid {
    fn default() -> Self {
        Grid { width: 10, height: 10, gap: 3, gap_color: BLACK, selected: None, selected_color: Some(BLUE) }
    }
}

impl Grid {
    pub fn new(width: i32, height: i32, gap: i32) -> Self {
        Grid { width, height, gap, gap_color: BLACK, selected: None, selected_color: Some(BLUE) }
    }

    // returns the (width, height) of each cell
    fn calculate_dimensions(&self) -> (i32, i32) {
        let screen_width: i32 = screen_width() as i32;
        let screen_height: i32 = screen_height() as i32;

        // in pixels
        let total_x_gap_space = (self.width - 1) * self.gap;
        let total_y_gap_space = (self.height - 1) * self.gap;

        let cell_width = (screen_width - total_x_gap_space) / self.width;
        let cell_height = (screen_height - total_y_gap_space) / self.height;

        (cell_width, cell_height)
    }

    pub fn draw(&self) {
        todo!()
    }
}

pub fn temp() {
    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
    draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
    draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);
}

#[cfg(test)]
mod tests {
    //use super::*;
    #[test]
    fn it_works() {
        todo!()
    }
}
