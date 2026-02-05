use macroquad::prelude::*;

#[derive(Clone)]
pub struct TrafficLight {
    pub state: String,
    next_state: String,
    pub timer: f32,
    clearing: bool,
}

const LIGHTS: [&str; 4] = ["down", "left", "up", "right"];

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            state: "down".to_string(),
            next_state: "left".to_string(),
            timer: 1.0,
            clearing: false,
        }
    }

    pub fn update_with_congestion(
        &mut self,
        dt: f32,
        up_count: usize,
        down_count: usize,
        left_count: usize,
        right_count: usize,
        capacity: usize,
        _is_intersection_clear: bool,
    ) {
        if self.timer > 0.0 {
            self.timer -= dt;
        }

        if self.clearing {
            if self.timer <= 0.0 {
                self.state = self.next_state.clone();
                self.clearing = false;

                let count = match self.state.as_str() {
                    "up" => up_count,
                    "down" => down_count,
                    "left" => left_count,
                    "right" => right_count,
                    _ => 0,
                };

                let ratio = (count as f32) / (capacity as f32);

                self.timer = if ratio > 0.4 {
                    2.0
                } else if count > 0 {
                    1.0
                } else {
                    0.5
                };
            }
        } else {
            if self.timer <= 0.0 {
                self.next_state =
                    self.calculate_priority(up_count, down_count, left_count, right_count);

                self.state = "ALL_RED".to_string();
                self.clearing = true;
                self.timer = 0.5;
            }
        }
    }

    fn calculate_priority(&self, up: usize, down: usize, left: usize, right: usize) -> String {
        let counts = [down, left, up, right].to_vec();

        let current_idx = LIGHTS.iter().position(|&x| x == self.state).unwrap_or(0);
        let mut filtered_counts = Vec::new();
        for (i, _) in counts.iter().enumerate() {
            if i == current_idx {
                continue;
            } else {
                filtered_counts.push(counts[i]);
            }
        }
        filtered_counts.sort();
        let mut next_idx = 0;
        for i in 0..counts.len() {
            if counts[i] == filtered_counts[filtered_counts.len() - 1] {
                next_idx = i;
                break;
            }
        }
        LIGHTS[next_idx].to_string()
    }

    pub fn get_state(&self) -> String {
        self.state.clone()
    }

    pub fn draw_lights(&self) {
        let w = screen_width();
        let h = screen_height();
        let cx = w / 2.0;
        let cy = h / 2.0;
        let gap = 60.0;
        let r = 8.0;

        let top_left_color = if self.state == "down" { GREEN } else { RED };
        let top_right_color = if self.state == "left" { GREEN } else { RED };
        let bottom_left_color = if self.state == "right" { GREEN } else { RED };
        let bottom_right_color = if self.state == "up" { GREEN } else { RED };

        draw_circle(cx - gap - 15.0, cy - gap - 15.0, r, top_left_color);
        draw_circle(cx + gap + 15.0, cy - gap - 15.0, r, top_right_color);
        draw_circle(cx - gap - 15.0, cy + gap + 15.0, r, bottom_left_color);
        draw_circle(cx + gap + 15.0, cy + gap + 15.0, r, bottom_right_color);

        // Debug Text
        let margin_x = 10.0;
        let margin_y = 10.0;
        let status_text = if self.clearing {
            "SWITCHING..."
        } else {
            &self.state
        };
        let status_color = if self.clearing { YELLOW } else { GREEN };

        draw_text(
            &format!("Timer: {:.1}s", self.timer),
            margin_x,
            margin_y + 20.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Active: {}", status_text.to_uppercase()),
            margin_x,
            margin_y + 50.0,
            20.0,
            status_color,
        );
    }
}
