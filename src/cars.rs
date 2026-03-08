use macroquad::prelude::*;

#[derive(Clone)]
pub struct Car {
    pub direction: String,
    pub width: i32,
    pub height: i32,
    pub cord: (f32, f32),
    pub velocity: f32,        // Replaced 'speed'
    pub target_velocity: f32, // Commanded by the smart intersection
    pub distance: f32,        // Distance left to leave the intersection
    pub time: f32,            // Time left to leave the intersection
    pub rotation: f32,
    pub spawn_time: f64,
}

impl Car {
    pub fn new(
        direction: String,
        width: i32,
        height: i32,
        cord: (f32, f32),
        rotation: f32,
    ) -> Self {
        Self {
            direction,
            width,
            height,
            cord,
            velocity: 400.0, // Default to cruising velocity
            target_velocity: 400.0,
            distance: 0.0,
            time: 0.0,
            rotation,
            spawn_time: get_time(),
        }
    }

    pub fn current_heading(&self) -> (f32, f32) {
        let (x, y) = self.cord;
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let mut dx = 0.0;
        let mut dy = 0.0;

        match self.direction.as_str() {
            "up_right" => {
                if y < cy + 80.0 {
                    dx = 1.0;
                } else {
                    dy = -1.0;
                }
            }
            "up_stright" => {
                dy = -1.0;
            }
            "up_left" => {
                if y < cy - 40.0 {
                    dx = -1.0;
                } else {
                    dy = -1.0;
                }
            }
            "right_right" => {
                if x < cx - 115.0 {
                    dx = 1.0;
                } else {
                    dy = 1.0;
                }
            }
            "right_stright" => {
                dx = 1.0;
            }
            "right_left" => {
                if x < cx + 5.0 {
                    dx = 1.0;
                } else {
                    dy = -1.0;
                }
            }
            "down_right" => {
                if y < cy - 125.0 {
                    dy = 1.0;
                } else {
                    dx = -1.0;
                }
            }
            "down_stright" => {
                dy = 1.0;
            }
            "down_left" => {
                if y < cy - 5.0 {
                    dy = 1.0;
                } else {
                    dx = 1.0;
                }
            }
            "left_right" => {
                if x > cx + 85.0 {
                    dx = -1.0;
                } else {
                    dy = -1.0;
                }
            }
            "left_stright" => {
                dx = -1.0;
            }
            "left_left" => {
                if x > cx - 35.0 {
                    dx = -1.0;
                } else {
                    dy = 1.0;
                }
            }
            _ => {}
        }
        (dx, dy)
    }



    pub fn get_rect(&self) -> Rect {
        let w = self.width as f32;
        let h = self.height as f32;

        // Macroquad rotates textures around their center point
        let cx = self.cord.0 + w / 2.0;
        let cy = self.cord.1 + h / 2.0;

        let (dx, dy) = self.current_heading();

        if dx.abs() > 0.1 {
            // Moving horizontally: visually, width and height are swapped
            Rect::new(cx - h / 2.0, cy - w / 2.0, h, w)
        } else {
            // Moving vertically: standard orientation
            Rect::new(cx - w / 2.0, cy - h / 2.0, w, h)
        }
    }

    pub fn get_radar(&self) -> Rect {
        // Base the radar directly off the accurately rotated physical bounding box
        let rect = self.get_rect();
        let (dx, dy) = self.current_heading();
        let radar_len = 100.0;

        if dx > 0.1 {
            // Moving right: radar projects from the right edge
            Rect::new(rect.x + rect.w, rect.y, radar_len, rect.h)
        } else if dx < -0.1 {
            // Moving left: radar projects from the left edge
            Rect::new(rect.x - radar_len, rect.y, radar_len, rect.h)
        } else if dy > 0.1 {
            // Moving down: radar projects from the bottom edge
            Rect::new(rect.x, rect.y + rect.h, rect.w, radar_len)
        } else if dy < -0.1 {
            // Moving up: radar projects from the top edge
            Rect::new(rect.x, rect.y - radar_len, rect.w, radar_len)
        } else {
            Rect::new(rect.x, rect.y, 0.0, 0.0)
        }
    }

    pub fn update(&mut self, dt: f32) {
        let accel = 500.0;
        let decel = 1500.0; // Very strong brakes for emergency AV stopping

        if self.velocity < self.target_velocity {
            self.velocity += accel * dt;
            if self.velocity > self.target_velocity {
                self.velocity = self.target_velocity;
            }
        } else if self.velocity > self.target_velocity {
            self.velocity -= decel * dt;
            if self.velocity < self.target_velocity {
                self.velocity = self.target_velocity;
            }
        }

        let step = self.velocity * dt;
        let (mut x, mut y) = self.cord;

        match self.direction.as_str() {
            "up_right" => {
                if y < screen_height() / 2.0 + 80.0 {
                    x += step;
                    self.rotation = 90.0;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "up_stright" => {
                y -= step;
            }
            "up_left" => {
                if y < screen_height() / 2.0 - 40.0 {
                    x -= step;
                    self.rotation = 270.0;
                } else {
                    y -= step;
                }
            }
            "right_right" => {
                if x < screen_width() / 2.0 - 115.0 {
                    x += step;
                } else {
                    y += step;
                    self.rotation = 180.0;
                }
            }
            "right_stright" => {
                x += step;
            }
            "right_left" => {
                if x < screen_width() / 2.0 + 5.0 {
                    x += step;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "down_right" => {
                if y < screen_height() / 2.0 - 125.0 {
                    y += step;
                } else {
                    x -= step;
                    self.rotation = 270.0;
                }
            }
            "down_stright" => {
                y += step;
            }
            "down_left" => {
                if y < screen_height() / 2.0 - 5.0 {
                    y += step;
                } else {
                    x += step;
                    self.rotation = 90.0;
                }
            }
            "left_right" => {
                if x > screen_width() / 2.0 + 85.0 {
                    x -= step;
                } else {
                    y -= step;
                    self.rotation = 0.0;
                }
            }
            "left_stright" => {
                x -= step;
            }
            "left_left" => {
                if x > screen_width() / 2.0 - 35.0 {
                    x -= step;
                } else {
                    y += step;
                    self.rotation = 180.0;
                }
            }
            _ => {}
        }

        self.cord = (x, y);

        // Physics updates: Calculate remaining distance and time
        match self.direction.split('_').next().unwrap_or("") {
            "up" => self.distance = y + 50.0,
            "down" => self.distance = screen_height() - y + 50.0,
            "left" => self.distance = x + 50.0,
            "right" => self.distance = screen_width() - x + 50.0,
            _ => self.distance = 0.0,
        }

        if self.velocity > 0.1 {
            self.time = self.distance / self.velocity;
        } else {
            self.time = f32::INFINITY; // Vehicle is stopped
        }
    }
}
