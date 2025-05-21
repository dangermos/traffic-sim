
use rayon::prelude::*;
use macroquad::prelude::*;
use cars_and_roads::{road::{Node, NodeID, generate_random_nodes, generate_random_roads}, Car, CarList, Road, RoadGraph, RoadID};
use render::{draw_car, draw_node, draw_road};


#[macroquad::main("Main Render")]
async fn main() {
    
    //// INIT ////
    
    set_fullscreen(true);


    let SCREEN_WIDTH: f32 = 1080.0;
    let SCREEN_HEIGHT: f32 = 1920.0;

    println!("Width: {}\nHeight: {}", SCREEN_WIDTH, SCREEN_HEIGHT);


    let CENTER: Vec2 = Vec2 {x: SCREEN_WIDTH / 2.0, y: SCREEN_HEIGHT / 2.0};

    // Nodes and Roads
    /* 
    let node1: Node = Node::new_node(NodeID(1), Vec2 { x: CENTER.x - 250.0, y: CENTER.y + 100.0 });
    let node2: Node = Node::new_node(NodeID(2), Vec2 { x: CENTER.x + 150.0, y: CENTER.y - 100.0 });
    let node3: Node = Node::new_node(NodeID(3), Vec2 { x: CENTER.x + 200.0, y: CENTER.y - 100.0 });

    let road1: Road = Road::new_road(RoadID(1), node1, node2, 100, 60.0, false);
    let road2: Road = Road::new_road(RoadID(2), node1, node3, 100, 60.0, false);
    let road3: Road = Road::new_road(RoadID(3), node2, node3, 100, 60.0, false);


    let mut road_graph: RoadGraph = RoadGraph::new(vec![road1, road2, road3].into(), 
                                                   vec![node1, node2, node3].into());

    */           

    let num_nodes = 5;

    let nodes = generate_random_nodes(num_nodes, SCREEN_WIDTH, SCREEN_HEIGHT);

    let num_roads = num_nodes;
    let roads = generate_random_roads(num_roads, &nodes);
    let road_graph = RoadGraph::new(roads.into(), nodes.into());


    let num_cars = num_roads;

    let mut cars: CarList = CarList::new(
        (0..num_cars)
            .map(|i| Car::new_on_road(RoadID(i % num_roads), &road_graph, 5.0))
            .collect()
    );

    // Arrays of game objects

    // Roads are accessed by ID using RoadGraph[id]
    


    /* println!("The roads are {:?}\nThe nodes are {:?}", 
    road_graph.get_roads().iter().map(|x| x.id).collect::<Vec<RoadID>>(),
    road_graph.get_nodes().iter().map(|x| x.id).collect::<Vec<NodeID>>());
    */

    panic!();
    //// Game Loop ////
    loop { 

        draw_fps();
        
        road_graph.get_roads().iter().for_each(|x| draw_road(x, WHITE));
        
        road_graph.get_nodes().iter().for_each(|x | draw_node(x));
        
        /* 

        for i in &mut cars {

            draw_car(&i, RED, true);

            i.move_car_on_road(get_frame_time() * 10.0, &road_graph);


            // println!("My Car ID is: {:?}\nI'm facing {} degrees!\nMy Current Position is {}\nI'm on road {:?}", i.get_id(), i.get_direction(), i.position, i.current_road);

        }
        */


        cars.get_cars_mut().par_iter_mut().for_each(|car| car.move_car_on_road(0.3, &road_graph));
        
        
        cars.get_cars().iter().for_each(|x | draw_car(x, RED, true));
        


        next_frame().await

    }
}



