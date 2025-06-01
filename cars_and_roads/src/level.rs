/// This just defines the Simulation I want to run.
/// Designed to be modular.
/// 

use crate::*;
use macroquad::math::Vec2;
use std::collections::HashMap;



/// Converts adjacency matrix to a dot file to be visualized.
pub fn adjacency_to_dot(adj: &HashMap<NodeID, Vec<(NodeID, RoadID)>>) -> String {
    let mut dot = String::new();
    use std::fmt::Write;
    
    writeln!(dot, "digraph G {{").unwrap();

    for (from, edges) in adj {
        for (to, road_id) in edges {
            writeln!(dot, "    {} -> {} [label=\"road {}\"];", from.0, to.0, road_id.0).unwrap();
        }
        if edges.is_empty() {
            writeln!(dot, "    {};", from.0).unwrap();
        }
    }

    writeln!(dot, "}}").unwrap();
    println!("{}", &dot);
    dot
}




pub struct Level {
    pub road_graph: RoadGraph,
    pub cars: CarList,

}


impl Level {

    pub fn sim1() -> Self {

    let screen_width: f32 = 1920.0;
    let screen_height: f32 = 1200.0;

    println!("Width: {}\nHeight: {}", screen_width, screen_height);


    let center: Vec2 = Vec2 {x: screen_width / 2.0, y: screen_height / 2.0};

    // Nodes and Roads
    
    let node1: Node = Node::new_node(NodeID(1), Vec2 { x: center.x - 450.0, y: center.y });
    let node2: Node = Node::new_node(NodeID(2), Vec2 { x: center.x + 150.0, y: center.y });
    let node3: Node = Node::new_node(NodeID(3), Vec2 { x: center.x + 150.0, y: center.y - 300.0 });
    let node4: Node = Node::new_node(NodeID(4), Vec2 { x: center.x - 600.0, y: center.y - 400.0 });

    let road1: Road = Road::new_road_with_curves(RoadID(0), node1, node2, 100, 60.0, 5);
    let road2: Road = Road::new_road(RoadID(1), node1, node3, 100, 60.0, );
    let road3: Road = Road::new_road(RoadID(2), node2, node3, 100, 60.0, );
    let road4: Road = Road::new_road_with_curves(RoadID(3), node3, node4, 30, 65.0, 5);


    let mut road_graph: RoadGraph = RoadGraph::new(vec![road1, road2, road3, road4].into(), 
                                                   vec![node1, node2, node3, node4].into());

    adjacency_to_dot(&road_graph.get_adjacency());

    let num_cars = 1;

    let cars: CarList = CarList::new(
        (0..num_cars)
            .map(|i| Car::new_on_road(RoadID(i as i32), &mut road_graph, 5.0))
            .collect()
    );


    Level {road_graph, cars}

    
    }








}



