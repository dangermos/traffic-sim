use std::{collections::HashMap, sync::{Arc, RwLock}};

use macroquad::{math::{Vec2}};
use rand::Rng;

use crate::{road, Car, CarID};



#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct NodeID (pub i32);

impl From<i32> for NodeID {
    fn from(value: i32) -> Self {
        NodeID(value)
    }
}

impl From<NodeID> for i32 {
    fn from(value: NodeID) -> Self {
        value.0
    }
}

impl ToString for NodeID {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub id: NodeID,
    pub position: Vec2,
}

impl Default for Node {
    fn default() -> Self {
        Node::new_node(NodeID(0), Vec2::ZERO)
    }
}

impl Node {
    pub fn new_node(id: NodeID, position: Vec2) -> Self {
        Node {
            id,
            position
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct RoadID (pub i32);
#[derive(Clone, Debug)]
/// A road is actually an edge between two Node objects
/// in the same way there are edges in a Directed Graph
pub struct Road {

    pub id: RoadID,
    pub from: Node,
    pub to: Node,
    pub length: f32,
    pub capacity: i32,
    pub vehicles_on: Vec<CarID>,
    pub num_vehicles_on: i32,
    pub speed_limit: f32,
    pub one_way: bool,
    pub traffic_density: f32,

    pub points: Vec<Vec2>, // this will expose any curves to the rendering function

}

/// Helper Function to populate points Vector
fn make_points(curves: i32, from_node: Node, to_node: Node) -> Vec<Vec2> { // Assuming NodeWithPos has position

    if curves == 0 {
        return Vec::new();
    }

    let mut rng = rand::rng(); // Standard way to get a RNG

    let from_pos = from_node.position;
    let to_pos = to_node.position;

    let min_x = from_pos.x.min(to_pos.x);
    let max_x = from_pos.x.max(to_pos.x);
    let min_y = from_pos.y.min(to_pos.y);
    let max_y = from_pos.y.max(to_pos.y);

    let mut intermediate_points: Vec<Vec2> = Vec::with_capacity(curves as usize);

    for _ in 0..curves {
        // Ensure max >= min for random_range. If they are equal, it might panic or behave unexpectedly.
        let x = if max_x > min_x { rng.random_range(min_x..max_x) } else { min_x };
        let y = if max_y > min_y { rng.random_range(min_y..max_y) } else { min_y };
        intermediate_points.push(Vec2::new(x, y));
    }

    // Sort points based on projection onto the from-to vector to make path more monotonic
    let line_vec = to_pos - from_pos;
    if line_vec.length_squared() > 0.001 { 
        intermediate_points.sort_by(|a, b| {
            let proj_a = (*a - from_pos).dot(line_vec);
            let proj_b = (*b - from_pos).dot(line_vec);
            proj_a.partial_cmp(&proj_b).unwrap_or(std::cmp::Ordering::Equal)
        });
    }


    intermediate_points
}

/// Generates 4 control points for a Bezier curve between start and end.
pub fn generate_bezier(start: Vec2, end: Vec2, curviness: f32) -> [Vec2; 4] {
    let dir = (end - start).normalize();
    let normal = Vec2::new(-dir.y, dir.x);

    let control1 = start + dir * 0.25 + normal * curviness;
    let control2 = start + dir * 0.75 - normal * curviness;

    [start, control1, control2, end]
}

pub fn bezier_point(points: [Vec2; 4], t: f32) -> Vec2 {
    let [p0, p1, p2, p3] = points;
    let u = 1.0 - t;
    u.powi(3) * p0 +
    3.0 * u.powi(2) * t * p1 +
    3.0 * u * t.powi(2) * p2 +
    t.powi(3) * p3
}

pub fn sample_bezier(points: [Vec2; 4], steps: usize) -> Vec<Vec2> {
    (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            bezier_point(points, t)
        })
        .collect()
}

impl Default for Road {
    fn default() -> Self {
        Road::new_road(RoadID(0), Node::default(), Node::default(), 0, 0.0)
    }
}

impl Road {
    pub fn new_road(id: RoadID, from: Node, to: Node, capacity: i32, speed_limit: f32,) -> Self {

        let num_vehicles_on = 0;
        let density = num_vehicles_on as f32 / capacity as f32;
        let length = from.position.distance(to.position);

        let one_way = rand::rng().random_range(1..=1000) < 200;


        let control = generate_bezier(from.position, to.position, 10.0);
        
        let points = sample_bezier(control, 30); // adjust step count for smoothness
        
        Road {
            id,
            from,
            to,
            length,
            capacity,
            vehicles_on: Vec::new(),
            num_vehicles_on,
            speed_limit,
            one_way,
            points,
            traffic_density: density,
        }
    }

    pub fn new_road_with_curves(id: RoadID, from: Node, to: Node, capacity: i32, speed_limit: f32, curviness: f32) -> Self {

        let num_vehicles_on = 0;
        let density = num_vehicles_on as f32 / capacity as f32;
        let length = from.position.distance(to.position);

        let one_way = rand::rng().random_range(1..=1000) < 200;


        let control = generate_bezier(from.position, to.position, curviness);

        let points = sample_bezier(control, 50); // adjust step count for smoothness
        


        Road {
            id,
            from,
            to,
            length,
            capacity,
            vehicles_on: Vec::new(),
            num_vehicles_on,
            speed_limit,
            one_way,
            points,
            traffic_density: density,
        }
    }
}

#[derive(Debug, Clone)]
/// A RoadGraph has an array representation of all the roads and nodes inserted into it.
/// 
/// It should also have an underlying Directed Graph for pathfinding algorithms.
pub struct RoadGraph {
    roads: HashMap<RoadID, Arc<RwLock<Road>>>,
    nodes: HashMap<NodeID, Node>,
    cars:  HashMap<CarID, Arc<RwLock<Car>>>,
    pub adjacency: HashMap<NodeID, Vec<(NodeID, RoadID)>>,
}


impl RoadGraph {
    /// Initialize a RoadGraph
    /// Takes an array of roads, nodes, and cars
    
    pub fn new(roads: Option<Vec<Road>>, nodes: Option<Vec<Node>>) -> Self {

        let mut road_map: HashMap<RoadID, Arc<RwLock<Road>>> = HashMap::new();
        let temp_roads = roads.unwrap_or_default();

        for road in temp_roads {
            road_map.insert(road.id, Arc::new(RwLock::new(road)));
        }

        let roads = road_map;


        let mut node_map: HashMap<NodeID, Node> = HashMap::new();

        let temp_nodes = nodes.unwrap_or_default();

        for node in temp_nodes {
            node_map.insert(node.id, node);
        }

        let nodes = node_map;
    

        let mut adjacency: HashMap<NodeID, Vec<(NodeID, RoadID)>> = HashMap::new();


        for (_id, road_arc) in &roads {
            let road = road_arc.read().unwrap();

            adjacency
                .entry(road.from.id) // from NodeID
                .or_default()
                .push((road.to.id, road.id)); // to (NodeID, using RoadID)
         }
        
        //println!("adj: {:?}", adjacency);


        let cars: HashMap<CarID, Arc<RwLock<Car>>> = HashMap::new();


        RoadGraph {
            roads,
            nodes,
            adjacency,
            cars,
        }

    }
    

    pub fn add_road(&mut self, road: Road) {
        self.roads.insert(road.id, Arc::new(RwLock::new(road)));
    }

    pub fn remove_road(&mut self, id: RoadID) {
        self.roads.remove(&id);
    }

    pub fn roads_to_iter(&self) -> impl Iterator<Item = &Arc<RwLock<Road>>> {
        self.roads.values()
    }

    pub fn get_roads(&self) -> &HashMap<RoadID, Arc<RwLock<Road>>>{
        &self.roads
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);
    }

    pub fn remove_node(&mut self, id: NodeID) {
        self.nodes.remove(&id);
    }

    pub fn nodes_to_iter(&self) -> impl Iterator<Item = &Node> {
        self.nodes.values()
    }

    pub fn get_nodes(&self) -> &HashMap<NodeID, Node> {
        &self.nodes
    }

    pub fn add_car(&mut self, car: Car) {
        self.cars.insert(car.get_id(), Arc::new(RwLock::new(car)));
    }

    pub fn remove_car(&mut self, id: CarID) {
        self.cars.remove(&id);
    }

    pub fn cars_to_iter(&self) -> impl Iterator<Item = &Arc<RwLock<Car>>> {
        self.cars.values()
    }

    pub fn get_cars(&self) -> &HashMap<CarID, Arc<RwLock<Car>>> {
        &self.cars
    }


    pub fn get_adjacency(&self) -> HashMap<NodeID, Vec<(NodeID, RoadID)>>{
        self.adjacency.clone()
    }



}




//////////////// TESTING FUNCTIONS /////////////////

pub fn generate_random_roads(num: i32, nodes: &[Node]) -> Vec<Road> {
    let mut rng = rand::rng();
    let mut roads = Vec::new();

    for i in 0..num {
        let from = nodes[rng.random_range(0..nodes.len())];
        let mut to = from;
        
        // ensure from ≠ to
        while to.id == from.id {
            to = nodes[rng.random_range(0..nodes.len())];
        }

        roads.push(Road::new_road(
            RoadID(i),
            from,
            to,
            rng.random_range(20..100),    // capacity
            rng.random_range(30.0..80.0), // speed_limit
        ));
    }

    roads
}

pub fn generate_random_nodes(num: i32, x_size: f32, y_size: f32) -> Vec<Node> {
    let mut rng = rand::rng();
    (0..num)
        .map(|i| {
            let x = rng.random_range(0.0..x_size);
            let y = rng.random_range(0.0..y_size);
            Node {
                id: NodeID(i),
                position: Vec2::new(x, y),
            }
        })
        .collect()
}

