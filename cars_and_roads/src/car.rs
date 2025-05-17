use macroquad::math::Vec2;
use rand::Rng;
use crate::RoadID;
pub type CarID = i32;
pub struct Car {
    // Public
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    // Private
    car_id: CarID,
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
            car_id: rand::rng().random::<i32>(),
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
}

pub struct CarList {
    cars: Vec<Car>,
}

impl CarList {
    pub fn new(cars: Option<Vec<Car>>) -> Self {
        if let Some(cars) = cars {
            CarList {
                cars
            }
        } else {
            CarList {
                cars: Vec::new(),
            }
        }
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
}

impl <'a>IntoIterator for &'a mut CarList {
    type Item = &'a mut Car;
    type IntoIter = std::slice::IterMut<'a, Car>;

    fn into_iter(self) -> Self::IntoIter {
        self.cars.iter_mut()
    }
}
