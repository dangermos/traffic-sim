use std::{collections::HashMap, ops::Index};

use macroquad::math::Vec2;
use rand::Rng;



#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct NodeID (pub i32);

impl From<i32> for NodeID {
    fn from(value: i32) -> Self {
        NodeID(value)
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Node {
    pub id: NodeID,
    pub position: Vec2,
}

impl Node {
    pub fn new_node(id: NodeID, position: Vec2) -> Self {
        Node {
            id,
            position
        }
    }
}


#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct RoadID (pub i32);
#[derive(Clone, Debug)]
/// A road is actually an edge between two Node objects
/// in the same way there are edges in a Directed Graph
pub struct Road {

    pub id: RoadID,
    from: Node,
    to: Node,
    length: f32,
    capacity: i32,
    vehicles_on: i32,
    pub speed_limit: f32,
    one_way: bool,
    traffic_density: f32,

    pub points: Vec<Vec2>, // this will expose any curves to the rendering function

}

/// Helper Function to populate points Vector
fn make_points(curves: i32, from: Node, to: Node) -> Vec<Vec2> {

    let mut rng = rand::rng();

    let min_x = from.position.x.min(to.position.x);
    let max_x = from.position.x.max(to.position.x);
    let min_y = from.position.y.min(to.position.y);
    let max_y = from.position.y.max(to.position.y);

    let mut points: Vec<Vec2> = Vec::with_capacity((curves + 2) as usize); // +2 for from/to


    for _ in 0..curves {
        let x = rng.random_range(min_x..=max_x);
        let y = rng.random_range(min_y..=max_y);
        points.push(Vec2::new(x, y));
    }

    points

}


impl Road {
    pub fn new_road(id: RoadID, from: Node, to: Node, capacity: i32, speed_limit: f32, one_way: bool) -> Self{

        let vehicles_on = 0;
        let density = vehicles_on as f32 / capacity as f32;
        let length = 0.0; // TODO

        let mut points: Vec<Vec2> = Vec::new();
        points.push(from.position);

        let num_curves = rand::rng().random_range(1..5);
        
        points.extend(make_points(num_curves, from, to));

        points.push(to.position);
        


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

/// A RoadGraph has an array representation of all the roads and nodes inserted into it.
/// 
/// It should also have an underlying Directed Graph for pathfinding algorithms.
pub struct RoadGraph {
    roads: Vec<Road>,
    nodes: Vec<Node>,
    adjacency: HashMap<NodeID, Vec<RoadID>>,
}


impl RoadGraph {
    /// Initialize a RoadGraph
    /// Takes an array of roads
    
    pub fn new(roads: Option<Vec<Road>>, nodes: Option<Vec<Node>>) -> Self {
        let roads = roads.unwrap_or_default();
        let nodes = nodes.unwrap_or_default();
        let mut adjacency: HashMap<NodeID, Vec<RoadID>> = HashMap::new();

        /* for road in &roads {
            adjacency
                .entry(road.from)
                .or_default()
                .push(road.id); */

        

        RoadGraph {
            roads,
            nodes,
            adjacency,
        }
    }
    

    pub fn add_road(&mut self, road: Road) {
        self.roads.push(road);
    }

    pub fn remove_road(&mut self, id: RoadID) {
        self.roads.retain_mut(|road| road.id != id);
    }

    pub fn get_roads(&self) -> &Vec<Road> {
        &self.roads
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn remove_node(&mut self, id: NodeID) {
        self.nodes.retain_mut(|node| node.id != id);
    }

    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }



}

/// This allows RoadGraph to be iterated on multiple times, not consuming it.
impl<'a> IntoIterator for &'a RoadGraph {
    type Item = &'a Road;
    type IntoIter = std::slice::Iter<'a, Road>;

    fn into_iter(self) -> Self::IntoIter {
        self.roads.iter()
    }
}


impl Index<RoadID> for RoadGraph {
    type Output = Road;
    fn index(&self, index: RoadID) -> &Self::Output {
        self.roads.iter()
                    .find(|road| road.id == index)
                    .expect("ID Not Found")
    }
}
impl Index<NodeID> for RoadGraph {
    type Output = Node;
    fn index(&self, index: NodeID) -> &Self::Output {
        self.nodes.iter()
                    .find(|node| node.id == index)
                    .expect("ID Not Found")
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
            true                       // one_way
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

