# Road Intersection Traffic Simulation

## Overview

This project is a real-time traffic simulation built in **Rust** using the **Macroquad** game engine. It visualizes a busy four-way intersection where vehicles must navigate traffic lights and traffic flow rules. The objective is to demonstrate a traffic control strategy that manages congestion and prevents collisions while allowing user interaction to spawn vehicles dynamically.

This simulation was developed to solve the "Traffic, traffic, traffic..." challenge, implementing specific rules for road layout, traffic light signaling, and vehicle behavior.

## Features

* **Interactive Simulation**: Users can manually spawn cars from different directions using keyboard controls.
* **Traffic Light System**: A dynamic 4-state traffic light system (North, South, East, West) that cycles automatically to manage flow.
* **Vehicle AI**:
    * **Routing**: Vehicles are assigned random routes (turning left, right, or going straight) indicated by their color.
    * **Collision Avoidance**: Logic to maintain safe distances between vehicles.
    * **Traffic Compliance**: Vehicles automatically stop at red lights and proceed when safe.
* **Visual Rendering**: Draws a complete road intersection with lane markings, dashed lines, and dynamic colored lights.

## Technologies Used

* **Language**: Rust
* **Graphics Library**: [Macroquad](https://macroquad.rs/) (Chosen for its simple and efficient 2D drawing capabilities)
* **Randomization**: `rand` crate for varied vehicle generation.

## Installation & Running

Ensure you have **Rust** and **Cargo** installed on your system.

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/AhmedBaid/road_intersection.git
    cd road_intersection
    ```

2.  **Run the project**:
    ```bash
    cargo run
    ```

## Controls

The simulation is controlled via the keyboard. You act as the "Traffic Generator," deciding when and where cars enter the city.

| Key | Action |
| :--- | :--- |
| **UP Arrow** | Spawn a car coming from the **South** (moving Up) |
| **DOWN Arrow** | Spawn a car coming from the **North** (moving Down) |
| **LEFT Arrow** | Spawn a car coming from the **East** (moving Left) |
| **RIGHT Arrow** | Spawn a car coming from the **West** (moving Right) |
| **C** or **Backspace** | Clear all cars from the screen |
| **Esc** | Exit the simulation |

> **Note:** The simulation includes safety logic that prevents you from "spamming" cars on top of each other. If a car is too close to the spawn point, a new one will not be created until there is a safe gap.

## Project Structure

The source code is modularized into several files for clarity:

* **`src/main.rs`**: The entry point of the application. It handles the main game loop, input detection (keyboard), and orchestrates the updates for cars and traffic lights.
* **`src/cars.rs`**: Defines the `Car` struct and its behavior.
    * Handles movement calculations (`update`).
    * Determines turning logic based on assigned colors (`get_root`).
    * Manages speed and directional logic.
* **`src/lights.rs`**: Manages the `TrafficLight` system.
    * Controls the timing and state switching (Green/Red) for different lanes.
    * Draws the traffic lights on the screen.
* **`src/draw_road.rs`**: Contains the drawing functions for the static environment, rendering the asphalt, lane dividers, and intersection geometry.
* **`src/dashed.rs`**: A utility helper for drawing the yellow dashed lines down the center of the roads.

## Simulation Logic

### 1. The Environment
The simulation renders two crossing roads. Each road supports one lane in each direction. The center of the screen is the "conflict zone" where paths merge and diverge.

### 2. Traffic Lights
The traffic lights operate on a timer-based state machine. The system cycles through four states, allowing traffic from one direction to proceed at a time while holding others:
1.  **Down** (Green for North-to-South traffic)
2.  **Left** (Green for East-to-West traffic)
3.  **Up** (Green for South-to-North traffic)
4.  **Right** (Green for West-to-East traffic)

### 3. Vehicles
* **Spawning**: When a key is pressed, a vehicle is instantiated with a random color.
* **Routes**: The color of the car determines its intended path (e.g., Red cars might turn left, Yellow might turn right) based on the specific logic defined in `cars.rs`.
* **Movement**: Vehicles move at a fixed speed. They check the state of the traffic light and the position of the car in front of them every frame. If the light is Red or the gap to the next car is unsafe, the vehicle halts.

## Future Improvements

* Add visual assets (sprites) for cars instead of rectangles.
* Implement "Yellow" light logic for smoother transitions.
* Add a "Random" spawn key (`R`) to automatically generate traffic from random directions.
* Display simulation statistics (total cars passed, average wait time).

---
*Project developed for the Traffic Simulation Challenge.*