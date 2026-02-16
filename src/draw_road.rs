use crate::dashed::*;
use macroquad::prelude::*;

pub const LANE_WIDTH: f32 = 30.0;
pub const GAP: f32 = LANE_WIDTH * 3.0; // 90.0 for 3 lanes

pub fn draw_road() {
    let w = screen_width();
    let h = screen_height();
    let cx = w / 2.0;
    let cy = h / 2.0;
    let thickness = 3.0;
    let curb_color = GOLD;
    let line_color = GRAY;

    // Draw grass/background
    clear_background(Color::from_rgba(4, 96, 85, 255));

    // Draw main black asphalt cross
    draw_rectangle(cx - GAP, 0.0, GAP * 2.0, h, BLACK);
    draw_rectangle(0.0, cy - GAP, w, GAP * 2.0, BLACK);

    // Draw Curb lines
    draw_line(cx - GAP, 0.0, cx - GAP, cy - GAP, thickness, curb_color);
    draw_line(cx + GAP, 0.0, cx + GAP, cy - GAP, thickness, curb_color);
    draw_line(cx + GAP, cy + GAP, cx + GAP, h, thickness, curb_color);
    draw_line(cx - GAP, cy + GAP, cx - GAP, h, thickness, curb_color);
    
    draw_line(0.0, cy - GAP, cx - GAP, cy - GAP, thickness, curb_color);
    draw_line(cx + GAP, cy - GAP, w, cy - GAP, thickness, curb_color);
    draw_line(0.0, cy + GAP, cx - GAP, cy + GAP, thickness, curb_color);
    draw_line(cx + GAP, cy + GAP, w, cy + GAP, thickness, curb_color);

    // Draw Middle yellow lines (Separating opposite directions)
    draw_line(cx, 0.0, cx, cy - GAP, thickness, YELLOW);
    draw_line(cx, cy + GAP, cx, h, thickness, YELLOW);
    draw_line(0.0, cy, cx - GAP, cy, thickness, YELLOW);
    draw_line(cx + GAP, cy, w, cy, thickness, YELLOW);

    // Draw lane separators (dashed)
    draw_dashed_line(vec2(cx + LANE_WIDTH, 0.0), vec2(cx + LANE_WIDTH, cy - GAP), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx + LANE_WIDTH*2.0, 0.0), vec2(cx + LANE_WIDTH*2.0, cy - GAP), 15.0, 10.0, 2.0, line_color);
    
    draw_dashed_line(vec2(cx - LANE_WIDTH, 0.0), vec2(cx - LANE_WIDTH, cy - GAP), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx - LANE_WIDTH*2.0, 0.0), vec2(cx - LANE_WIDTH*2.0, cy - GAP), 15.0, 10.0, 2.0, line_color);

    draw_dashed_line(vec2(cx + LANE_WIDTH, cy + GAP), vec2(cx + LANE_WIDTH, h), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx + LANE_WIDTH*2.0, cy + GAP), vec2(cx + LANE_WIDTH*2.0, h), 15.0, 10.0, 2.0, line_color);

    draw_dashed_line(vec2(cx - LANE_WIDTH, cy + GAP), vec2(cx - LANE_WIDTH, h), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx - LANE_WIDTH*2.0, cy + GAP), vec2(cx - LANE_WIDTH*2.0, h), 15.0, 10.0, 2.0, line_color);

    draw_dashed_line(vec2(0.0, cy - LANE_WIDTH), vec2(cx - GAP, cy - LANE_WIDTH), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(0.0, cy - LANE_WIDTH*2.0), vec2(cx - GAP, cy - LANE_WIDTH*2.0), 15.0, 10.0, 2.0, line_color);
    
    draw_dashed_line(vec2(0.0, cy + LANE_WIDTH), vec2(cx - GAP, cy + LANE_WIDTH), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(0.0, cy + LANE_WIDTH*2.0), vec2(cx - GAP, cy + LANE_WIDTH*2.0), 15.0, 10.0, 2.0, line_color);

    draw_dashed_line(vec2(cx + GAP, cy - LANE_WIDTH), vec2(w, cy - LANE_WIDTH), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx + GAP, cy - LANE_WIDTH*2.0), vec2(w, cy - LANE_WIDTH*2.0), 15.0, 10.0, 2.0, line_color);
    
    draw_dashed_line(vec2(cx + GAP, cy + LANE_WIDTH), vec2(w, cy + LANE_WIDTH), 15.0, 10.0, 2.0, line_color);
    draw_dashed_line(vec2(cx + GAP, cy + LANE_WIDTH*2.0), vec2(w, cy + LANE_WIDTH*2.0), 15.0, 10.0, 2.0, line_color);
}