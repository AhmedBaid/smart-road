use macroquad::input::KeyCode::{Down, Left, Right, Up};
use macroquad::{prelude::*, rand::gen_range};
use std::default::Default;

mod stats;
use stats::*;
mod car;
use car::*;
mod draw_road;
use draw_road::*;
mod dashed;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Smart Road"),
        window_height: 1200,
        window_width: 1200,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut statistics: Stats = Stats {
        total_cars: 0,
        best_time: 999999999.,
        worst_time: 0.,
        best_velocity: 0.,
        worst_velocity: 999999999.,
        collisions: 0,
        close_calls: 0,
    };

    let mut is_escaped: bool = false;
    let mut is_exit: bool = false;
    let mut is_paused = false;
    let mut is_random = false;
    let mut is_debug_mode = false;
    
    // Make sure you have a "car.png" in an "assets" folder next to Cargo.toml
    let car_texture: Texture2D = load_texture("assets/car.png").await.unwrap_or(Texture2D::empty());
    let mut cars: Vec<Car> = Vec::new();
    let core_intersection = Rect::new(503., 520., 180., 180.);

    loop {
        if is_key_pressed(KeyCode::Escape) {
            if is_exit {
                std::process::exit(0);
            } else {
                is_escaped = true;
                is_exit = true;
            }
        }
        if is_key_pressed(KeyCode::P) { is_paused = !is_paused; }
        if is_key_pressed(KeyCode::D) { is_debug_mode = !is_debug_mode; }

        if is_escaped {
            clear_background(DARKGRAY);
            statistics.draw_endgame();
        } else if is_paused {
            clear_background(Color::from_rgba(30, 30, 30, 255));
            draw_road();
            
            if is_debug_mode {
                draw_rectangle(core_intersection.x, core_intersection.y, core_intersection.w, core_intersection.h, Color::new(0.5, 0.5, 0., 0.1));
            }

            cars.iter().for_each(|car| car.draw_all_components(&car_texture, is_debug_mode));
            draw_text("Press P to continue", 430., 600., 40., GOLD);
        } else {
            // INPUT
            if is_key_pressed(Left) { Car::spawn_if_can(&mut cars, vec!["RU", "RL", "RD"][gen_range(0, 3)], "West"); } 
            else if is_key_pressed(Up) { Car::spawn_if_can(&mut cars, vec!["DU", "DL", "DR"][gen_range(0, 3)], "North"); } 
            else if is_key_pressed(Down) { Car::spawn_if_can(&mut cars, vec!["UL", "UD", "UR"][gen_range(0, 3)], "South"); } 
            else if is_key_pressed(Right) { Car::spawn_if_can(&mut cars, vec!["LU", "LR", "LD"][gen_range(0, 3)], "East"); } 
            else if is_key_pressed(KeyCode::R) { is_random = !is_random; }

            if is_random && gen_range(0, 100) < 5 { // Throttle random spawning slightly
                let random_direction = vec!["West", "North", "South", "East"][gen_range(0, 4)];
                match random_direction {
                    "West" => Car::spawn_if_can(&mut cars, vec!["RU", "RL", "RD"][gen_range(0, 3)], random_direction),
                    "North" => Car::spawn_if_can(&mut cars, vec!["DU", "DL", "DR"][gen_range(0, 3)], random_direction),
                    "South" => Car::spawn_if_can(&mut cars, vec!["UL", "UD", "UR"][gen_range(0, 3)], random_direction),
                    "East" => Car::spawn_if_can(&mut cars, vec!["LU", "LR", "LD"][gen_range(0, 3)], random_direction),
                    _ => {}
                }
            }

            // UPDATE
            cars.retain(|car| {
                if &*car.current_direction == "West" && car.car_rect.x < 100. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    false
                } else if &*car.current_direction == "North" && car.car_rect.y < 100. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    false
                } else if &*car.current_direction == "South" && car.car_rect.y > 1050. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    false
                } else if &*car.current_direction == "East" && car.car_rect.x + car.car_size.long_edge > 1100. {
                    car.check_for_best_or_worst_time(&mut statistics);
                    statistics.total_cars += 1;
                    false
                } else {
                    true
                }
            });

            let mut temp_cars = cars.clone();
            cars.iter().for_each(|car| car.check_for_collision(&mut temp_cars, &mut statistics));

            let temp_cars = cars.clone();
            cars.iter_mut().for_each(|car| car.communicate_with_intersection(&temp_cars, &core_intersection));

            let temp_cars = cars.clone();
            for (car_index, car) in cars.iter_mut().enumerate() {
                car.update_radar(car_index, &temp_cars);
            }

            cars.iter_mut().for_each(|car| car.adjust_current_speed());

            let mut temp_cars = cars.clone();
            cars.iter_mut().filter(|car| !car.waiting_flag).for_each(|car| car.move_one_step_if_no_collide(&mut temp_cars, &mut statistics));

            let temp_cars = cars.clone();
            cars.iter_mut().for_each(|car| car.turn_if_can(&temp_cars));

            // RENDER
            clear_background(Color::from_rgba(30, 30, 30, 255));
            draw_road(); // Using your drawn road!

            if is_debug_mode {
                draw_rectangle(core_intersection.x, core_intersection.y, core_intersection.w, core_intersection.h, Color::new(0.5, 0.5, 0., 0.1));
            }

            cars.iter().for_each(|car| car.draw_all_components(&car_texture, is_debug_mode));
            statistics.draw_ingame();
        }
        next_frame().await;
    }
}