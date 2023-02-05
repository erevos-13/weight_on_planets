#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use crate::structs::planet::Planet;
use crate::utils::constants::{JUPYTER_GRAVITY, MARS_GRAVITY, MERCURY_GRAVITY, MOON_GRAVITY, NEPTUNE_GRAVITY, SATURN_GRAVITY, URANUS_GRAVITY, VENUS_GRAVITY};
mod structs;
mod utils;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn find_weight_on_planet(planet_id: i8, weight_number: f32) -> String  {
  let result =  match planet_id {
        1 => {
            let planet_moon = Planet::new("Moon".to_string(), MOON_GRAVITY, 1);
            let w = planet_moon.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_moon.get_name(),
                w
            )
        },
        2 => {
            let planet_mercury = Planet::new("Mercury".to_string(), MERCURY_GRAVITY, 2);
            let w = planet_mercury.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_mercury.get_name(),
                w
            )
        },
        3 => {
            let planet_venus = Planet::new("Venus".to_string(), VENUS_GRAVITY, 3);
            let w = planet_venus.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_venus.get_name(),
                w
            )
        },
        4 => {
            let planet_mars = Planet::new("Mars".to_string(), MARS_GRAVITY, 4);
            let w = planet_mars.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_mars.get_name(),
                w
            )
        },
        5 => {
            let planet_jupyter = Planet::new("Jupyter".to_string(), JUPYTER_GRAVITY, 5);
            let w = planet_jupyter.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_jupyter.get_name(),
                w
            )
        },
        6 => {
            let planet_neptune = Planet::new("Neptune".to_string(), NEPTUNE_GRAVITY, 6);
            let w = planet_neptune.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_neptune.get_name(),
                w
            )
        },
        7 => {
            let planet_uranus = Planet::new("Uranus".to_string(), URANUS_GRAVITY, 7);
            let w = planet_uranus.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_uranus.get_name(),
                w
            )
        },
        8 => {
            let planet_saturn = Planet::new("Saturn".to_string(), SATURN_GRAVITY, 8);
            let w = planet_saturn.cal_gravity_on_planet(weight_number);
            format!(
                "You weight in {} is: {}",
                planet_saturn.get_name(),
                w
            )
        },
        _ => format!("The planet number you selected is not exist"),
    };
    result
}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_weight_on_planet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
