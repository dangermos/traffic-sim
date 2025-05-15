
use macroquad::math::Vec2;
use rand::Rng;
pub type RoadID = i32;

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
