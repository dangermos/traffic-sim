use std::{collections::HashMap, ops::{Index, IndexMut}};

use macroquad::{math::{Vec2}};
use rand::Rng;



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
    pub vehicles_on: i32,
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



impl Road {
    pub fn new_road(id: RoadID, from: Node, to: Node, capacity: i32, speed_limit: f32) -> Self{

        let vehicles_on = 0;
        let density = vehicles_on as f32 / capacity as f32;
        let length = from.position.distance(to.position);

        let one_way = rand::rng().random_range(1..=1000) < 200;


        let mut points: Vec<Vec2> = Vec::new();
        points.push(from.position);

        let num_curves = 3;
        
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

    pub fn new_road_with_curves(id: RoadID, from: Node, to: Node, capacity: i32, speed_limit: f32, curves: i32) 
    -> Self {

        let vehicles_on = 0;
        let density = vehicles_on as f32 / capacity as f32;
        let length = from.position.distance(to.position);

        let one_way = rand::rng().random_range(1..=1000) < 200;


        let mut points: Vec<Vec2> = Vec::new();
        points.push(from.position);

        
        points.extend(make_points(curves, from, to));

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
    pub adjacency: HashMap<NodeID, Vec<(NodeID, RoadID)>>,
}


impl RoadGraph {
    /// Initialize a RoadGraph
    /// Takes an array of roads
    
    pub fn new(roads: Option<Vec<Road>>, nodes: Option<Vec<Node>>) -> Self {

        let roads = roads.unwrap_or_default();
        let nodes = nodes.unwrap_or_default();
        


        let mut adjacency: HashMap<NodeID, Vec<(NodeID, RoadID)>> = HashMap::new();

        // h.into_iter().for_each(|(key, val)| adjacency.insert(key, val); );


         for road in &roads {
            adjacency
                .entry(road.from.id) // from NodeID
                .or_default()
                .push((road.to.id, road.id)); // to (NodeID, using RoadID)
         }
        
        println!("adj: {:?}", adjacency);

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

    pub fn get_adjacency(&self) -> HashMap<NodeID, Vec<(NodeID, RoadID)>>{
        self.adjacency.clone()
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
                    .expect(&format!("ID {:?} Not Found", index))
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






impl IndexMut<RoadID> for RoadGraph {

    fn index_mut(&mut self, index: RoadID) -> &mut Self::Output {
        self.roads.iter_mut()
        .find(|road| road.id == index)
        .expect("ID Not Found")  
    }
}

impl IndexMut<NodeID> for RoadGraph {
    fn index_mut(&mut self, index: NodeID) -> &mut Self::Output {
        self.nodes.iter_mut()
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

