use macroquad::prelude::*;
use std::f32::consts::PI;
use crate::draw_road::{GAP, LANE_WIDTH};

#[derive(Clone, Copy, PartialEq)]
pub enum Route { Right, Straight, Left }

#[derive(Clone, Copy, PartialEq)]
pub enum Origin { North, South, East, West }

#[derive(Clone)]
pub struct Car {
    pub origin: Origin,
    pub route: Route,
    pub cord: Vec2,
    pub rotation: f32, // For animation/asset rotation
    pub color: Color,
    
    // Physics tracking
    pub target_speed: f32,
    pub current_speed: f32,
    pub distance_traveled: f32,
    pub time_active: f32,
    
    // Logic state
    pub passed_intersection: bool,
    pub turn_center: Vec2,
    pub turn_radius: f32,
    pub turn_start_angle: f32,
    pub in_intersection: bool,
}

impl Car {
    pub fn new(origin: Origin, route: Route, w: f32, h: f32) -> Self {
        let cx = w / 2.0;
        let cy = h / 2.0;
        
        let mut cord = Vec2::ZERO;
        let mut rotation = 0.0;
        
        // Spawn locations based on Origin and Route (3 distinct lanes)
        match origin {
            Origin::South => {
                rotation = -PI / 2.0; // Facing up
                cord.y = h + 50.0;
                cord.x = match route {
                    Route::Left => cx + LANE_WIDTH * 0.5,
                    Route::Straight => cx + LANE_WIDTH * 1.5,
                    Route::Right => cx + LANE_WIDTH * 2.5,
                };
            }
            Origin::North => {
                rotation = PI / 2.0; // Facing down
                cord.y = -50.0;
                cord.x = match route {
                    Route::Left => cx - LANE_WIDTH * 0.5,
                    Route::Straight => cx - LANE_WIDTH * 1.5,
                    Route::Right => cx - LANE_WIDTH * 2.5,
                };
            }
            Origin::East => {
                rotation = PI; // Facing left
                cord.x = w + 50.0;
                cord.y = match route {
                    Route::Left => cy + LANE_WIDTH * 0.5,
                    Route::Straight => cy + LANE_WIDTH * 1.5,
                    Route::Right => cy + LANE_WIDTH * 2.5,
                };
            }
            Origin::West => {
                rotation = 0.0; // Facing right
                cord.x = -50.0;
                cord.y = match route {
                    Route::Left => cy - LANE_WIDTH * 0.5,
                    Route::Straight => cy - LANE_WIDTH * 1.5,
                    Route::Right => cy - LANE_WIDTH * 2.5,
                };
            }
        }

        Self {
            origin, route, cord, rotation,
            color: Color::from_rgba(rand::gen_range(50, 255), rand::gen_range(50, 255), rand::gen_range(50, 255), 255),
            target_speed: 120.0,
            current_speed: 120.0,
            distance_traveled: 0.0,
            time_active: 0.0,
            passed_intersection: false,
            turn_center: Vec2::ZERO,
            turn_radius: 0.0,
            turn_start_angle: 0.0,
            in_intersection: false,
        }
    }

    pub fn update(&mut self, dt: f32, cx: f32, cy: f32) {
        self.time_active += dt;
        
        // Acceleration / Deceleration smoothing
        self.current_speed += (self.target_speed - self.current_speed) * 5.0 * dt;
        let step = self.current_speed * dt;
        self.distance_traveled += step;

        let in_x_bounds = self.cord.x > cx - GAP && self.cord.x < cx + GAP;
        let in_y_bounds = self.cord.y > cy - GAP && self.cord.y < cy + GAP;
        self.in_intersection = in_x_bounds && in_y_bounds;

        if !self.in_intersection && !self.passed_intersection {
            // Driving straight towards intersection
            let dir = vec2(self.rotation.cos(), self.rotation.sin());
            self.cord += dir * step;
        } else if self.in_intersection {
            self.passed_intersection = true; // Once inside, mark as passed so it exits later
            
            if self.route == Route::Straight {
                let dir = vec2(self.rotation.cos(), self.rotation.sin());
                self.cord += dir * step;
            } else {
                // Initialize turn variables once
                if self.turn_radius == 0.0 {
                    self.setup_turn(cx, cy);
                }

                // Move along the arc
                let angular_velocity = self.current_speed / self.turn_radius;
                
                // Determine rotation direction based on route
                let angle_step = if self.route == Route::Right {
                    angular_velocity * dt // Turn clockwise
                } else {
                    -angular_velocity * dt // Turn counter-clockwise
                };

                self.rotation += angle_step;
                
                // Keep moving along the arc using trigonometry
                let turn_angle = self.rotation + if self.route == Route::Right { PI/2.0 } else { -PI/2.0 };
                self.cord.x = self.turn_center.x + turn_angle.cos() * self.turn_radius;
                self.cord.y = self.turn_center.y + turn_angle.sin() * self.turn_radius;
            }
        } else if self.passed_intersection {
            // Driving straight away from intersection
            let dir = vec2(self.rotation.cos(), self.rotation.sin());
            self.cord += dir * step;
        }
    }

    fn setup_turn(&mut self, cx: f32, cy: f32) {
        // Calculate the pivot point for smooth curves
        match self.origin {
            Origin::South => {
                if self.route == Route::Right {
                    self.turn_center = vec2(cx + GAP, cy + GAP);
                    self.turn_radius = LANE_WIDTH * 0.5;
                } else {
                    self.turn_center = vec2(cx - GAP, cy + GAP);
                    self.turn_radius = GAP + LANE_WIDTH * 0.5;
                }
            }
            Origin::North => {
                if self.route == Route::Right {
                    self.turn_center = vec2(cx - GAP, cy - GAP);
                    self.turn_radius = LANE_WIDTH * 0.5;
                } else {
                    self.turn_center = vec2(cx + GAP, cy - GAP);
                    self.turn_radius = GAP + LANE_WIDTH * 0.5;
                }
            }
            Origin::East => {
                if self.route == Route::Right {
                    self.turn_center = vec2(cx + GAP, cy - GAP);
                    self.turn_radius = LANE_WIDTH * 0.5;
                } else {
                    self.turn_center = vec2(cx + GAP, cy + GAP);
                    self.turn_radius = GAP + LANE_WIDTH * 0.5;
                }
            }
            Origin::West => {
                if self.route == Route::Right {
                    self.turn_center = vec2(cx - GAP, cy + GAP);
                    self.turn_radius = LANE_WIDTH * 0.5;
                } else {
                    self.turn_center = vec2(cx - GAP, cy - GAP);
                    self.turn_radius = GAP + LANE_WIDTH * 0.5;
                }
            }
        }
    }
}