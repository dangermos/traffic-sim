use macroquad::math::Vec2;
use rand::Rng;
use crate::road::NodeID;
use crate::{RoadID, RoadGraph, Road};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};




#[derive(Copy, Clone, PartialEq)]
/// State is used for pathfinding algorithms
struct State {
    node: NodeID,
    cost: f32,         // g(n)
    est_total: f32,    // f(n) = g(n) + h(n)
}

impl State {
    fn new(node: NodeID, cost: f32, est_total: f32) -> Self {
        State { node, cost, est_total }
    }
}

impl Eq for State {}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior
        other.est_total.partial_cmp(&self.est_total).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



pub fn a_star(start_node: NodeID, goal_node: NodeID, graph: &RoadGraph, debug: bool) -> Vec<RoadID> {
    
    let mut open = BinaryHeap::new();
    let mut came_from: HashMap<NodeID, NodeID> = HashMap::new();
    let mut cost_so_far: HashMap<NodeID, f32> = HashMap::new();

    let goal_pos = graph[goal_node].position;
    let start_pos = graph[start_node].position;

    open.push(State::new(
        start_node,
        0.0,
        start_pos.distance(goal_pos),
    ));

    cost_so_far.insert(start_node, 0.0);

    while let Some(State { node: current, cost, .. }) = open.pop() {
        if current == goal_node {
            if debug {
                println!("Reached goal node {:?}", current);
            }

            // Reconstruct node path
            let mut node_path = vec![current];
            let mut curr = current;
            while let Some(&prev) = came_from.get(&curr) {
                node_path.push(prev);
                curr = prev;
            }
            node_path.reverse();

            // Convert to road path
            let mut road_path = Vec::new();
            for win in node_path.windows(2) {
                let from = win[0];
                let to = win[1];
                if let Some(edges) = graph.adjacency.get(&from) {
                    if let Some(&(_, road_id)) = edges.iter().find(|&&(n, _)| n == to) {
                        road_path.push(road_id);
                    } else if debug {
                        println!("⚠ No road found from {:?} to {:?}", from, to);
                    }
                }
            }

            if debug {
                println!("Final node path: {:?}", node_path);
                println!("Final road path: {:?}", road_path);
            }

            return road_path;
        }

        if let Some(neighbors) = graph.adjacency.get(&current) {
            for &(neighbor, road_id) in neighbors {
                let road = &graph[road_id];
                if road.one_way {
                    let dir = (road.to.position - road.from.position).normalize();
                    let travel = (graph[neighbor].position - graph[current].position).normalize();
                    if dir.dot(travel) <= 0.0 {
                        continue; // wrong direction
                    }
                }

                let base_cost = road.length.max(1.0);
                let density_penalty = 1.0 + road.traffic_density * 3.0;
                let weight = base_cost * density_penalty;

                let new_cost = cost + weight;

                if new_cost < *cost_so_far.get(&neighbor).unwrap_or(&f32::INFINITY) {
                    cost_so_far.insert(neighbor, new_cost);
                    came_from.insert(neighbor, current);
                    let est = new_cost + graph[neighbor].position.distance(goal_pos);
                    open.push(State::new(neighbor, cost, est));
                }
            }
        }
    }

    if debug {
        println!("No path found from {:?} to {:?}", start_node, goal_node);
    }

    vec![] // no path
}






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
    path: Vec<RoadID>,

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
            segment_index: 0,
            path: Vec::new(),
        }
    }

    /// Spawns a car on the specified road of a RoadGraph.
    pub fn new_on_road(road: RoadID, road_graph: &mut RoadGraph, velocity: f32) -> Self {

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

        road_graph[road].vehicles_on += 1;

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
            segment_index: 0,
            path: Vec::new(),
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

    /// Moves Car to end of road it is currently on
    /// 
    /// Returns true when at end of road
    // In Car impl:
    pub fn move_car_on_road(&mut self, dt: f32, road_graph: &RoadGraph) -> bool {

        let road = &road_graph[self.current_road];
        let points = &road.points;

        const MIN_SEGMENT_LENGTH_FOR_HEADING_SQ: f32 = 0.01; 

        // If segment_index is invalid or road has < 2 points for any meaningful segment.
        if points.len() < 2 {
            return true; // Cannot form a segment, consider it "done".
        }
        
        if self.segment_index >= points.len() - 1 {
            // Already at the end of the last segment.
            if !points.is_empty() {
                self.position = points[points.len() - 1]; // Ensure snapped to the very end.
            }
            return true; // Done with the road.
        }

        // Target point of the current segment.
        let segment_target_point = points[self.segment_index + 1];
        let segment_start_point = points[self.segment_index]; // Current base of segment

        // Ensure heading is correctly set for the current segment.
        let current_segment_direction = (segment_target_point - segment_start_point).normalize_or_zero();
        if current_segment_direction.length_squared() > MIN_SEGMENT_LENGTH_FOR_HEADING_SQ {
            // Only update heading if the segment is not tiny and heading is not already aligned.
            // This avoids issues if car was teleported or segment is very small.
        } else if (segment_target_point - self.position).length_squared() < MIN_SEGMENT_LENGTH_FOR_HEADING_SQ {
            // Segment is tiny, and we are already basically at its target point.
            // Force advance to next segment.
            self.position = segment_target_point;
            self.segment_index += 1;

            // Update heading
            if self.segment_index < points.len() - 1 {
                let next_segment_start = points[self.segment_index];
                let next_segment_target = points[self.segment_index + 1];
                let next_direction = (next_segment_target - next_segment_start).normalize_or_zero();
                if next_direction.length_squared() > MIN_SEGMENT_LENGTH_FOR_HEADING_SQ {
                    self.heading = next_direction.to_angle();
                }
            }
            return self.segment_index >= points.len() - 1; // Check if done
        }


        let vector_car_to_target = segment_target_point - self.position;
        let distance_to_target = vector_car_to_target.length();
        
        let travel_this_frame = self.velocity * dt;

        if travel_this_frame >= distance_to_target && distance_to_target > 0.001 { // This accounts for any wild floating point things
            // Car reaches or passes the target point of the current segment
            self.position = segment_target_point; // Snap to target
            self.segment_index += 1;

            // Check if done with the entire road
            if self.segment_index >= points.len() - 1 {
                return true; // Done with road and returns successful
            } else {
                // Not done with road, so prepare for the next segment
                // Update heading for the new current segment
                let new_segment_start = points[self.segment_index]; // This is segment_target_point
                let new_segment_target = points[self.segment_index + 1];
                let new_direction = (new_segment_target - new_segment_start).normalize_or_zero();
                if new_direction.length_squared() > MIN_SEGMENT_LENGTH_FOR_HEADING_SQ {
                    self.heading = new_direction.to_angle();
                } else {
                    // New segment is also tiny, keep old heading or let next iteration handle it.
                }
                return false; // Advanced to next segment, not done with road
            }
        } else if distance_to_target <= 0.001 { // Already at (or very close to) the target
            self.position = segment_target_point; // Snap for precision
            self.segment_index += 1;
            if self.segment_index >= points.len() - 1 {
                return true; // Done with road
            } else {
                let new_segment_start = points[self.segment_index];
                let new_segment_target = points[self.segment_index + 1];
                let new_direction = (new_segment_target - new_segment_start).normalize_or_zero();
                if new_direction.length_squared() > MIN_SEGMENT_LENGTH_FOR_HEADING_SQ {
                    self.heading = new_direction.to_angle();
                }
                return false;
            }
        }
        else {
            // Car moves along the current segment but does not reach its end
            // Move using the segment's fixed direction (derived from current self.heading,
            // which should have been set when this segment was entered).
            let move_direction = Vec2::from_angle(self.heading); // Assumes heading is correct for current segment
            self.position += move_direction * travel_this_frame;
            
            // self.heading = (segment_target_point - self.position).normalize_or_zero().to_angle();
            // will actually cause oscillations! 
            // It was my initial step and I spent a good amount of time debugging it. 
        }

        false // Not done with current segment of road
    }
    
    

    /// Moves car from starting road to inputted destination
    /// 
    /// Uses the A* algorithm 
    pub fn move_car_to_destination(&mut self, road_graph: &RoadGraph, destination: NodeID, dt: f32, debug: bool) {
        // check if car done with its own road
        let done = self.move_car_on_road(dt, road_graph);

        if debug {
            println!(
            "[Step] ID: {:?} | Pos: {:.1},{:.1} | Seg: {} / {}",
            self.car_id,
            self.position.x,
            self.position.y,
            self.segment_index,
            road_graph[self.current_road].points.len() - 1
            ); 
        }
    
        // Car is at end of road, and destination.id matches current_road.end.id

        if self.path.is_empty()
            && self.segment_index >= road_graph[self.current_road].points.len() - 1
            && road_graph[self.current_road].to.id == destination
        {
            if debug {
                println!("✅ Fully arrived at destination {:?}", destination);
            }
            return;
        }
    
        // Runs A* again when road is finished.
        // Can potentially be modified to provide on-the-fly rerouting, as in it will suggest another road before car finishes its own.
        if self.path.is_empty() {
            let road = &road_graph[self.current_road];
    
            if self.segment_index < road.points.len() - 1 {
                // Still moving on current road, wait before routing
                return;
            }
    
            let start_node = road.to.id;
            self.path = a_star(start_node, destination, road_graph, debug);
    
            if debug {
                println!("📍 Rerouted from node {:?} to {:?}, path: {:?}", start_node, destination, self.path);
            }
    
            if self.path.first() == Some(&self.current_road) {
                self.path.remove(0);
            }
        }
    
        // Moves to next road in path if exists. This is the only function that can move cars to different roads. 
        if done {
            if let Some(next_road) = self.path.first().copied() {
                self.path.remove(0);
                self.current_road = next_road;
                self.segment_index = 0;
    
                let new_road = &road_graph[self.current_road];
                let points = &new_road.points;
    
                let dist_to_start = (points[0] - self.position).length();
                if dist_to_start < 2.0 {
                    self.position = points[0]; // Snap to road start
                } else {
                    println!("❌ Car jumped to road {:?} with dist {:.2}. Rejecting.", self.current_road, dist_to_start);
                    self.path.clear(); // Invalidate bad path
                    return;
                }
    
                if points.len() >= 2 {
                    let dir = (points[1] - points[0]).normalize_or_zero();
                    if dir.length_squared() > 0.0 {
                        self.heading = dir.to_angle();
                    }
                }
            }
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
