use macroquad::math::Vec2;

type RoadID = i32;


pub enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}



pub struct Car {

    // Public
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,





    // Private
    current_road: RoadID,



    // For Rendering
    x_size: i32,
    y_size: i32,
    heading: Direction


}


impl Car {
    pub fn new(position: Vec2, velocity: Vec2, facing: Direction) -> Self {
        Car { 
            position,
            velocity,
            acceleration: Vec2::ZERO,
            current_road: 0,

            x_size: 5,
            y_size: 10,
            heading: facing
        }
    }




}