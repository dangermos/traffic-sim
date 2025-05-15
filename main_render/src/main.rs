use car::Car;
use car::Direction;
use macroquad::prelude::*;


/// This function exists because the direction the car is facing changes its rendering properties with area.
fn draw_car(car: Car) {

    
    


}


#[macroquad::main("Main Render")]



async fn main() {
    
    // INIT

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;


    let map  = [0; ((SCREEN_HEIGHT / 10) * (SCREEN_WIDTH / 10)) as usize];


    let car1 = Car::new(Vec2::ZERO, Vec2::ZERO, Direction::EAST);
    





    clear_background(WHITE);


    
    



    // Game Loop
    loop { 

        draw_rectangle(car1.position.x, car1.position.y, car1, h, color);

    }

}
