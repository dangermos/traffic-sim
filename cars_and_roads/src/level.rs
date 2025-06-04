/// This just defines the Simulation I want to run.
/// Designed to be modular.
/// 

use crate::*;
use macroquad::math::Vec2;
use ::rand::random_range;
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
}


impl Level {

    pub fn sim1(device: String) -> Self {

    let screen_width = if device == "laptop" {1920.0} else {1200.0};
    let screen_height = if device == "laptop" {1200.0} else {1920.0};

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

    let num_roads = road_graph.get_roads().len() - 1;

    adjacency_to_dot(&road_graph.get_adjacency());

    let num_cars = 7;

    let cars: Vec<Car> = 
        (0..num_cars)
            .map(|i| Car::new_on_road(None, RoadID(i & num_roads as i32), &mut road_graph, 5.0, NodeID(4)))
            .collect();

    for car in cars {
        road_graph.add_car(car);
    }

    Level {road_graph}

    
    }


    pub fn sim2(device: String) -> Self {
        let screen_width = if device == "laptop" {1920.0} else {1200.0};
        let screen_height = if device == "laptop" {1200.0} else {1920.0};
        let center = Vec2 { x: screen_width / 2.0, y: screen_height / 2.0 };

        println!("Width: {}\nHeight: {}", screen_width, screen_height);

        // === Nodes ===
        let node_top = Node::new_node(NodeID(1), Vec2 { x: center.x, y: center.y - 300.0 });
        let node_right = Node::new_node(NodeID(2), Vec2 { x: center.x + 300.0, y: center.y });
        let node_bottom = Node::new_node(NodeID(3), Vec2 { x: center.x, y: center.y + 300.0 });
        let node_left = Node::new_node(NodeID(4), Vec2 { x: center.x - 300.0, y: center.y });
        let node_center = Node::new_node(NodeID(5), center);

        // === Roads ===
        let road_top_center = Road::new_road(RoadID(0), node_top, node_center, 100, 60.0);
        let road_center_right = Road::new_road_with_curves(RoadID(1), node_center, node_right, 100, 60.0, 3);
        let road_right_bottom = Road::new_road(RoadID(2), node_right, node_bottom, 100, 60.0);
        let road_bottom_center = Road::new_road_with_curves(RoadID(3), node_bottom, node_center, 100, 60.0, 3);
        let road_center_left = Road::new_road(RoadID(4), node_center, node_left, 100, 60.0);
        let road_left_top = Road::new_road_with_curves(RoadID(5), node_left, node_top, 100, 60.0, 3);

        let mut road_graph = RoadGraph::new(
            vec![
                road_top_center,
                road_center_right,
                road_right_bottom,
                road_bottom_center,
                road_center_left,
                road_left_top,
            ].into(),
            vec![node_top, node_right, node_bottom, node_left, node_center].into(),
        );

        adjacency_to_dot(&road_graph.get_adjacency());

        // === Cars using your syntax ===
        let cars: Vec<Car> = 
            (0..6)
                .map(|i| {
                    // match the same goals as earlier sim2 for variety
                    let goals = [
                        NodeID(3), // bottom
                        NodeID(4), // left
                        NodeID(1), // top
                        NodeID(2), // right
                        NodeID(1), // top
                        NodeID(3), // bottom
                    ];
                    Car::new_on_road(None, RoadID(i), &mut road_graph, 5.0, goals[i as usize])
                })
                .collect();

        for car in cars {
            road_graph.add_car(car);
        }
        Level { road_graph }
    }   

    pub fn sim3() -> Self {

        let screen_width: f32 = 1920.0;
        let screen_height: f32 = 1200.0;
        let center = Vec2 { x: screen_width / 2.0, y: screen_height / 2.0 };

        println!("Width: {}\nHeight: {}", screen_width, screen_height);

        // === Nodes ===
        let node_start = Node::new_node(NodeID(1), Vec2 { x: center.x - 500.0, y: center.y });
        let node_top   = Node::new_node(NodeID(2), Vec2 { x: center.x + 300.0, y: center.y - 200.0 });
        let node_mid   = Node::new_node(NodeID(3), Vec2 { x: center.x + 300.0, y: center.y });
        let node_bot   = Node::new_node(NodeID(4), Vec2 { x: center.x + 300.0, y: center.y + 200.0 });

        // === Roads ===
        let road_top = Road::new_road_with_curves(RoadID(0), node_start, node_top, 100, 55.0, 4);
        let road_mid = Road::new_road(RoadID(1), node_start, node_mid, 100, 55.0);
        let road_bot = Road::new_road_with_curves(RoadID(2), node_start, node_bot, 100, 55.0, 4);

        let mut road_graph = RoadGraph::new(
            vec![road_top, road_mid, road_bot].into(),
            vec![node_start, node_top, node_mid, node_bot].into(),
        );

        adjacency_to_dot(&road_graph.get_adjacency());

        let num_cars = 3;

        let cars: Vec<Car> = {
        let goals = [NodeID(2), NodeID(3), NodeID(4)];
        
            (0..num_cars)
                .map(|i| Car::new_on_road(None, RoadID(i), &mut road_graph, 5.0, goals[i as usize]))
                .collect()
            };

        for car in cars {
            road_graph.add_car(car);
        }

        Level { road_graph }
    }

    pub fn sim_roundabout(device: String) -> Level {

        let screen_width = if device == "laptop" {1920.0} else {1200.0};
        let screen_height = if device == "laptop" {1200.0} else {1920.0};

        let center = Vec2::new(screen_width / 2.0, screen_height / 2.0);
        let radius = 200.0;
    
        // 4-way roundabout (N, E, S, W)
        let node_n = Node::new_node(NodeID(0), center + Vec2::new(0.0, -radius * 2.0));
        let node_e = Node::new_node(NodeID(1), center + Vec2::new(radius * 2.0, 0.0));
        let node_s = Node::new_node(NodeID(2), center + Vec2::new(0.0, radius * 2.0));
        let node_w = Node::new_node(NodeID(3), center + Vec2::new(-radius * 2.0, 0.0));
    
        // Inner roundabout nodes (clockwise)
        let node_rn = Node::new_node(NodeID(4), center + Vec2::new(0.0, -radius));
        let node_re = Node::new_node(NodeID(5), center + Vec2::new(radius, 0.0));
        let node_rs = Node::new_node(NodeID(6), center + Vec2::new(0.0, radius));
        let node_rw = Node::new_node(NodeID(7), center + Vec2::new(-radius, 0.0));
    
        let nodes = vec![node_n, node_e, node_s, node_w, node_rn, node_re, node_rs, node_rw];
    
        let roads = vec![
            Road::new_road(RoadID(0), node_n, node_rn, 40, 30.0),
            Road::new_road(RoadID(1), node_rn, node_re, 40, 30.0),
            Road::new_road(RoadID(2), node_re, node_rs, 40, 30.0),
            Road::new_road(RoadID(3), node_rs, node_rw, 40, 30.0),
            Road::new_road(RoadID(4), node_rw, node_rn, 40, 30.0), // loop back
    
            Road::new_road(RoadID(5), node_rs, node_s, 40, 30.0),
            Road::new_road(RoadID(6), node_re, node_e, 40, 30.0),
            Road::new_road(RoadID(7), node_rw, node_w, 40, 30.0),
        ];
    
        let mut road_graph = RoadGraph::new(Some(roads), Some(nodes));
    
        const NUM_CARS: i32 = 5;

        let fin_nodes = [2, 1, 3];
        let speed = random_range(3.0..5.0);

        let cars: Vec<Car> = 
                (0..=NUM_CARS)
                .into_iter()
                .map(|x| Car::new_on_road(None, RoadID(0), &mut road_graph, speed, NodeID(fin_nodes[x as usize % fin_nodes.len()])))
                .collect(); // N to E)
    


        for car in cars {
            road_graph.add_car(car);
        }
        Level { road_graph }
    }
    
    
    

}
    
    



