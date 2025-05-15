use car::Car;
use macroquad::prelude::*;


fn draw_car(car: &Car, color: Color) {

    let width = car.get_width();
    let height = car.get_height();

    let angle_rad = car.get_direction().to_radians();
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    // Define the rectangle corners relative to center
    let corners = [
        Vec2::new(-half_w, -half_h), // top-left
        Vec2::new( half_w, -half_h), // top-right
        Vec2::new( half_w,  half_h), // bottom-right
        Vec2::new(-half_w,  half_h), // bottom-left
    ];

    // Rotate and translate each corner
    let rotated: Vec<Vec2> = corners.iter().map(|p| {
        let rotated_x = p.x * angle_rad.cos() - p.y * angle_rad.sin();
        let rotated_y = p.x * angle_rad.sin() + p.y * angle_rad.cos();
        car.position + Vec2::new(rotated_x, rotated_y)
    }).collect();

    // Draw it as two triangles
    draw_triangle(rotated[0], rotated[1], rotated[2], color);
    draw_triangle(rotated[2], rotated[3], rotated[0], color);



    

}


#[macroquad::main("Main Render")]



async fn main() {
    
    // INIT

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 600;
    const CENTER: Vec2 = Vec2 {x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32};

    let map  = [0; ((SCREEN_HEIGHT / 10) * (SCREEN_WIDTH / 10)) as usize];


    let mut car1 = Car::new(CENTER, Vec2::ZERO, 0.0);
    





    clear_background(WHITE);


    
    



    // Game Loop
    loop { 

        draw_car(&car1, RED);
        car1.rotate_car(0.001);
        next_frame().await

    }




}
