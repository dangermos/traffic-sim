use macroquad::prelude::*;
use cars_and_roads::{Car, Road, road::make_points};
use render::{draw_car, draw_road};
// fn draw_road


#[macroquad::main("Main Render")]



async fn main() {
    
    // INIT

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;
    const CENTER_X: f32 = (SCREEN_WIDTH / 2) as f32;
    const CENTER_Y: f32 = (SCREEN_HEIGHT / 2) as f32;
    const CENTER: Vec2 = Vec2 {x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32};


    let mut car1 = Car::new(CENTER, Vec2::ZERO, 0.0);
    

    // make road points
    let curves = make_points(CENTER, 4);

    let road1: Road = Road::new_road(1, 1, 2, 10.0, 100, 60.0, curves, false);






    

    // Game Loop
    loop { 
        draw_road(&road1, WHITE);

        draw_car(&car1, RED);
        car1.rotate_car(0.0);
        car1.velocity = Vec2 {x: 0.0, y: 1.0};
        


        next_frame().await

    }




}
