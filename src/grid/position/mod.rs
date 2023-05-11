// where should the grid/text/thing be displayed?
// this enum is meant to be generic
// that is, it should work for x and y values
// the goal:
// if a y value is centered, the text or grid or whatever will just be centered
// if a x value is centered, the text or grid or whatever will just be centered
//
// this enum and all of its functions assume that, when drawing something,
// the (x, y) pair provided represents the top left corner of the thing being drawn
// (similar to the draw_rect function and unlike the draw_text function)
#[derive(Clone, Copy, Default)]
pub enum Position {
    #[default]
    Start, // left or top
    End, // right or bottom
    Center, // middle (either way)
    Pixels(f32) // + means right or down and - means left or up
}

pub fn as_pixels(position: Position, width_or_height_of_thing: f32, width_or_height_of_screen: f32) -> f32 {
    match position {
        Position::Start => 0.0,
        Position::End => width_or_height_of_screen - width_or_height_of_thing,
        Position::Center => (width_or_height_of_screen - width_or_height_of_thing)/2.0,
        Position::Pixels(offset) => offset,
    }
}

impl From<f32> for Position {
    fn from(value: f32) -> Self {
        Position::Pixels(value)
    }
}

