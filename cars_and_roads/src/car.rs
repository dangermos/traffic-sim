use macroquad::math::Vec2;
use crate::RoadID;

pub struct Car {
    // Public
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    // Private
    current_road: RoadID,

    // For Rendering
    width: f32,
    height: f32,
    center: Vec2,
    heading: f32,
}

impl Car {
    pub fn new(position: Vec2, velocity: Vec2, heading: f32) -> Self {

            let width= 5.0;
            let height= 15.0;

        Car { 
            position,
            velocity,
            acceleration: Vec2::ZERO,
            current_road: 0,
            width,
            height,
            center: Vec2 { x: width / 2.0, y: height / 2.0 }, 
            heading,
        }
    }

    pub fn get_direction(&self) -> f32 {
        self.heading
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }

    pub fn get_center(&self) -> Vec2 {
        self.center
    }

    pub fn rotate_car(&mut self, rotation: f32) -> () {
        
        self.heading += rotation
        
    }
}