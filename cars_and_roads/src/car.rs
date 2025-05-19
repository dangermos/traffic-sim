use macroquad::math::Vec2;
use rand::Rng;
use crate::{RoadID, RoadGraph};
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CarID (pub i32);


impl CarID {
    fn new_rand() -> Self{
        CarID(rand::rng().random_range(1..1000))
    } 
}

impl From<i32> for CarID {
    fn from(value: i32) -> Self {
        CarID(value)
    }
}

pub struct Car {
    // Public
    pub position: Vec2,
    pub velocity: f32,
    pub acceleration: Vec2,
    pub segment_index: usize,

    // Private
    car_id: CarID,
    pub current_road: RoadID,

    // For Rendering
    width: f32,
    height: f32,
    center: Vec2,
    heading: f32,
}

impl Car {
    pub fn new(position: Vec2, velocity: f32, heading: f32) -> Self {
            let width= 5.0;
            let height= 15.0;
            let center = Vec2 { x: width / 2.0, y: height / 2.0 }; 

        Car { 
            position,
            velocity,
            acceleration: Vec2::ZERO,
            current_road: RoadID(0),
            width,
            height,
            center, 
            heading,
            car_id: CarID::new_rand(),
            segment_index: 0
        }
    }

    /// Spawns a car on the specified road of a RoadGraph.
    pub fn new_on_road(road: RoadID, road_graph: &RoadGraph, velocity: f32) -> Self {

        // this is the first point of the road.
        let position = road_graph[road].points[0];
        // safe because for a road to exist it must have at least 2 points.
        let next_pos = road_graph[road].points[1];

        // the heading should be the angle needed to point from the first point to the second,
        // which ends up being the unit vector of both x y vectors. 
        // this is included in the Vec2 struct as Normalize()

        let heading = (next_pos - position).normalize_or_zero().to_angle();
        let width= 5.0;
        let height= 15.0;
        let center = Vec2 { x: width / 2.0, y: height / 2.0 }; 


        Car { 
            position,
            velocity,
            acceleration: Vec2::ZERO,
            car_id: CarID::new_rand(),
            current_road: road,
            width,
            height,
            center, 
            heading,
            segment_index: 0
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

    pub fn get_id(&self) -> CarID {
        self.car_id
    }

    pub fn rotate_car(&mut self, rotation: f32) {
        
        if self.heading == 360.0 {
            self.heading = 0.0;
        }

        self.heading += rotation
        
    }

    pub fn move_car(&mut self, position: Vec2) {
        self.position += position
    }


    pub fn move_car_on_road(&mut self, dt: f32, road_graph: &RoadGraph) {
        let road = &road_graph[self.current_road];
        let points = &road.points;
    
        if self.segment_index + 1 >= points.len() {
            //self.current_road = road_graph.get_roads().iter().next().map(|x| x.id).unwrap();
            //self.segment_index = 0;
            return;
        }
    
        let from = points[self.segment_index];
        let to = points[self.segment_index + 1];
    
        let direction = (to - from).normalize();
        let speed = self.velocity; // scalar
        let movement = direction * speed * dt;
    
        self.position += movement;
    
        // Updates car direction
        if direction.length_squared() > 0.0 {
            self.heading = direction.to_angle();
        }
    
        // Check if we reached or passed the target point
        if (to - self.position).length_squared() < 1.0 {
            self.position = to;
            self.segment_index += 1;
        }
    }
    
    

}



pub struct CarList {
    cars: Vec<Car>,
}

impl CarList {
    pub fn new(cars: Vec<Car>) -> Self {
        CarList { cars }
    }
    pub fn add_car(&mut self, car: Car) {
        self.cars.push(car);
    }
    pub fn remove_car(&mut self, car_id: CarID) {
        self.cars.retain(|car| car.car_id != car_id)
    }
    pub fn get_cars(&self) -> &Vec<Car> {
        &self.cars
    }
    pub fn get_cars_mut(&mut self) ->&mut Vec<Car> {
        &mut self.cars
    }
}

impl <'a>IntoIterator for &'a mut CarList {
    type Item = &'a mut Car;
    type IntoIter = std::slice::IterMut<'a, Car>;

    fn into_iter(self) -> Self::IntoIter {
        self.cars.iter_mut()
    }
}
