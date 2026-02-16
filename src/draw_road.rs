use crate::dashed::*;
use macroquad::prelude::*;

pub fn draw_road() {
    let screen_width = screen_width();
    let screen_height = screen_height();
    let tickness = 4.0;
    
    // Adjusted gap to 130 to perfectly wrap the coordinates inside car.rs
    let gap = 130.0; 
    let color1 = GOLD;

    // Background road rectangles
    draw_rectangle((screen_width / 2.0) - gap, 0.0, gap * 2.0, screen_height, BLACK);
    draw_rectangle(0.0, screen_height / 2.0 - gap, screen_width, gap * 2.0, BLACK);

    // Curbs
    draw_line((screen_width / 2.0) - gap, (screen_height / 2.0) - gap, (screen_width / 2.0) - gap, 0.0, tickness, color1);
    draw_line((screen_width / 2.0) + gap, (screen_height / 2.0) - gap, (screen_width / 2.0) + gap, 0.0, tickness, color1);
    draw_line((screen_width / 2.0) + gap, (screen_height / 2.0) - gap, screen_width, (screen_height / 2.0) - gap, tickness, color1);
    draw_line((screen_width / 2.0) + gap, (screen_height / 2.0) + gap, screen_width, (screen_height / 2.0) + gap, tickness, color1);
    draw_line((screen_width / 2.0) + gap, (screen_height / 2.0) + gap, (screen_width / 2.0) + gap, screen_height, tickness, color1);
    draw_line((screen_width / 2.0) - gap, (screen_height / 2.0) + gap, (screen_width / 2.0) - gap, screen_height, tickness, color1);
    draw_line((screen_width / 2.0) - gap, (screen_height / 2.0) + gap, 0.0, (screen_height / 2.0) + gap, tickness, color1);
    draw_line((screen_width / 2.0) - gap, (screen_height / 2.0) - gap, 0.0, (screen_height / 2.0) - gap, tickness, color1);
    
    // Mid lines
    draw_dashed_middle_lines(0.0);
}