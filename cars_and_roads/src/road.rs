use std::ops::Index;

use macroquad::math::Vec2;
use rand::Rng;
pub type RoadID = i32;

#[derive(Clone, Debug)]
pub struct Road {

    pub id: RoadID,
    from: RoadID,
    to: RoadID,
    length: f32,
    capacity: i32,
    vehicles_on: i32,
    pub speed_limit: f32,
    one_way: bool,
    traffic_density: f32,

    pub points: Vec<Vec2>, // this will expose any curves to the rendering function

}

impl Road {
    pub fn new_road(id: RoadID, from: RoadID, to: RoadID,  length: f32, capacity: i32, speed_limit: f32, points: Vec<Vec2>, one_way: bool) -> Self{

        let vehicles_on = 0;
        let density = vehicles_on as f32 / capacity as f32;


        Road {
            id,
            from,
            to,
            length,
            capacity,
            vehicles_on,
            speed_limit,
            one_way,
            points,
            traffic_density: density,
        }
    }

}

pub struct RoadList {
    roads: Vec<Road>,
}

impl RoadList {
    /// Initialize a roadlist
    /// Takes an array of roads
    pub fn new(road: Option<Vec<Road>>) -> Self {
        
        if let Some(road) = road {

        RoadList {
            roads: Vec::from_iter(road)
            }
        }
        else {
            RoadList {
                roads: Vec::new(),
            }
        }
    
    }

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }

    pub fn get_roads(&self) -> &Vec<Road> {
        &self.roads
    }
}

/// This allows RoadList to be iterated on multiple times, not consuming it.
impl<'a> IntoIterator for &'a RoadList {
    type Item = &'a Road;
    type IntoIter = std::slice::Iter<'a, Road>;

    fn into_iter(self) -> Self::IntoIter {
        self.roads.iter()
    }
}

impl Index<RoadID> for RoadList {
    type Output = Road;
    fn index(&self, index: RoadID) -> &Self::Output {
        self.roads.iter()
                    .find(|road| road.id == index)
                    .expect("ID Not Found")
    }
}

/// Helper Function to populate points Vector
pub fn make_points(center: Vec2, curves: i32) -> Vec<Vec2> {
    let mut rng = rand::rng();


    println!("Center is {}", center);


    let lower_y: f32 = (center.y * 2.0) - 10.0;
    let higher_y: f32 = 10.0;
    let left_x: f32 = 10.0;
    let right_x: f32 = (center.x * 2.0) - 10.0;

    let mut points: Vec<Vec2> = Vec::with_capacity(curves as usize);


    for _ in 0..curves {
        points.push(Vec2 { x: rng.random_range(left_x..=right_x), y: rng.random_range(higher_y..=lower_y) });
    }

    points

}
