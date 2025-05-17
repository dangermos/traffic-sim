use macroquad::prelude::*;
use cars_and_roads::{car::CarList, road::{make_points, RoadList}, Car, Road};
use render::{draw_car, draw_road};


#[macroquad::main("Main Render")]
async fn main() {
    
    //// INIT ////

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;
    const CENTER: Vec2 = Vec2 {x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32};


    let car1 = Car::new(CENTER, Vec2::ZERO, 0.0);
    let car2 = Car::new(Vec2 { x: CENTER.x + 1.0, y: CENTER.y - 10.0 }, Vec2::ZERO, 0.0);

    // make road points
    let curves = make_points(CENTER, 3);
    let curves2 = make_points(CENTER, 2);

    let road1: Road = Road::new_road(1, 1, 2, 30.0, 100, 60.0, curves, false);

    let road2: Road = Road::new_road(2, 1, 2, 10.0, 100, 60.0, curves2, false);


    // Arrays of game objects

    // Roads are accessed by ID using RoadList[id]
    let mut roads: RoadList = RoadList::new(vec![road1, road2].into());
    
    let mut cars: CarList = CarList::new(vec![car1, car2].into());


    

    //// Game Loop ////
    loop { 

        draw_fps();
        for i in &roads {
            draw_road(&i, WHITE);
        }

        for i in &mut cars {
            i.rotate_car(1.0);
            draw_car(&i, RED);

            println!("My Car ID is: {}\nI'm facing {} degrees!", i.get_id(), i.get_direction())

        }

        
        
        
        


        next_frame().await

    }
}



