use macroquad::prelude::*;
mod cars;
mod dashed;
mod draw_road;
use cars::*;
use draw_road::*;

// To satisfy the 3 distinct velocities requirement
const VELOCITY_SLOW: f32 = 60.0;
const VELOCITY_MED: f32 = 120.0;
const VELOCITY_FAST: f32 = 180.0;

#[derive(Default)]
struct Stats {
    passed_vehicles: usize,
    max_speed: f32,
    min_speed: f32,
    max_time: f32,
    min_time: f32,
    close_calls: usize,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Smart Intersection".to_string(),
        window_width: 1000,
        window_height: 1000,
        window_resizable: false,
        ..Default::default()
    }
}

fn can_spawn(cars: &[Car], origin: Origin, route: Route, w: f32, h: f32) -> bool {
    let dummy = Car::new(origin, route, w, h);
    for car in cars {
        if car.cord.distance(dummy.cord) < 80.0 {
            return false;
        }
    }
    true
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut cars: Vec<Car> = Vec::new();
    let mut stats = Stats { min_speed: 999.0, min_time: 999.0, ..Default::default() };
    let mut auto_spawn = false;
    let mut last_spawn_time = get_time();

    // Create a generic car texture for rotation animation
    let car_img = Image::gen_image_color(40, 20, WHITE);
    let car_texture = Texture2D::from_image(&car_img);

    loop {
        let dt = get_frame_time();
        draw_road();

        if is_key_pressed(KeyCode::Escape) {
            break; // Exit to stats screen
        }

        if is_key_pressed(KeyCode::R) {
            auto_spawn = !auto_spawn;
        }

        // --- SPAWN LOGIC ---
        let mut try_spawn = |origin: Origin| {
            let routes = [Route::Left, Route::Straight, Route::Right];
            let route = routes[rand::gen_range(0, 3)];
            if can_spawn(&cars, origin, route, screen_width(), screen_height()) {
                cars.push(Car::new(origin, route, screen_width(), screen_height()));
            }
        };

        if is_key_pressed(KeyCode::Up) { try_spawn(Origin::South); }
        if is_key_pressed(KeyCode::Down) { try_spawn(Origin::North); }
        if is_key_pressed(KeyCode::Right) { try_spawn(Origin::West); }
        if is_key_pressed(KeyCode::Left) { try_spawn(Origin::East); }

        if auto_spawn && get_time() - last_spawn_time > 0.5 {
            let origins = [Origin::North, Origin::South, Origin::East, Origin::West];
            try_spawn(origins[rand::gen_range(0, 4)]);
            last_spawn_time = get_time();
        }

        // --- SMART INTERSECTION MANAGEMENT ---
        for i in 0..cars.len() {
            let mut requested_speed = VELOCITY_FAST;
            let my_cord = cars[i].cord;

            for j in 0..cars.len() {
                if i == j { continue; }
                let dist = my_cord.distance(cars[j].cord);

                // Check Close Calls (Safety Distance Violation)
                if dist < 45.0 {
                    stats.close_calls += 1;
                }

                // If approaching someone in front, slow down (Car Following)
                if dist < 80.0 && !cars[i].in_intersection {
                    requested_speed = VELOCITY_SLOW;
                }

                // Collision Avoidance inside/near intersection
                if (cars[i].in_intersection || cars[j].in_intersection) && dist < 100.0 {
                     if i > j { requested_speed = VELOCITY_MED; } 
                     else { requested_speed = VELOCITY_SLOW; }
                }
            }
            cars[i].target_speed = requested_speed;
            cars[i].update(dt, screen_width() / 2.0, screen_height() / 2.0);
        }

        // --- RENDER & REMOVE ---
        cars.retain(|car| {
            let cx = screen_width() / 2.0;
            let cy = screen_height() / 2.0;
            
            draw_texture_ex(
                &car_texture, 
                car.cord.x - 20.0, car.cord.y - 10.0, 
                car.color, 
                DrawTextureParams { rotation: car.rotation, ..Default::default() }
            );

            // Calculate physics dynamically: v = d / t
            let calculated_velocity = if car.time_active > 0.0 { car.distance_traveled / car.time_active } else { 0.0 };

            // Remove car if it's far off screen
            if car.cord.x < -100.0 || car.cord.x > screen_width() + 100.0 || 
               car.cord.y < -100.0 || car.cord.y > screen_height() + 100.0 {
                
                // Record Stats upon exit
                stats.passed_vehicles += 1;
                if calculated_velocity > stats.max_speed { stats.max_speed = calculated_velocity; }
                if calculated_velocity < stats.min_speed { stats.min_speed = calculated_velocity; }
                if car.time_active > stats.max_time { stats.max_time = car.time_active; }
                if car.time_active < stats.min_time { stats.min_time = car.time_active; }
                false
            } else {
                true
            }
        });

        next_frame().await;
    }

    // --- STATISTICS SCREEN LOOP ---
    loop {
        clear_background(DARKGRAY);
        let center_x = screen_width() / 2.0 - 200.0;
        let mut y = 200.0;
        
        draw_text("SIMULATION FINISHED - STATISTICS", center_x - 50.0, 100.0, 40.0, GOLD);
        
        let stats_text = [
            format!("Total Vehicles Passed: {}", stats.passed_vehicles),
            format!("Max Velocity (d/t): {:.2} px/s", stats.max_speed),
            format!("Min Velocity (d/t): {:.2} px/s", if stats.min_speed == 999.0 { 0.0 } else { stats.min_speed }),
            format!("Max Time to Pass: {:.2} s", stats.max_time),
            format!("Min Time to Pass: {:.2} s", if stats.min_time == 999.0 { 0.0 } else { stats.min_time }),
            format!("Close Calls Detected: {}", stats.close_calls / 2), // Div 2 because both cars log the close call
        ];

        for text in stats_text.iter() {
            draw_text(text, center_x, y, 30.0, WHITE);
            y += 50.0;
        }

        draw_text("Press 'Q' to Quit", center_x + 50.0, y + 50.0, 30.0, RED);
        if is_key_pressed(KeyCode::Q) { break; }
        
        next_frame().await;
    }
}