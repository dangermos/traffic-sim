
use rayon::prelude::*;
use macroquad::{prelude::*};
use cars_and_roads::level::Level;
use cars_and_roads::Car;
use render::*;



#[macroquad::main("Main Render")]
async fn main() {

    
    //// INIT ////
    
    set_fullscreen(true);


    // let level = Level::sim1();

     let level = Level::sim_roundabout("pc".to_string(), 30); // 'pc' is for a vertical 1080p display, laptop is for a normal 1080p display (but mine is 1920x1200) 

    //let level = Level::sim1();

    let mut road_graph = level.road_graph;


    
    //// Game Loop ////
    loop { 

        draw_fps();
        

        // Render //
        draw_roads(&mut road_graph, false);
        road_graph.nodes_to_iter().for_each(|x| draw_node(x, true));
        road_graph.cars_to_iter().for_each(|x | draw_car(&x.read().unwrap(), false));



        // Simulation // 
        let frame_time = get_frame_time();
        road_graph.get_cars().par_iter().for_each(|(_id, car)| {car.write().unwrap().move_car_to_destination(&road_graph,  frame_time * 40.0, false);});

        //road_graph.get_cars().par_iter_mut().for_each(|(id, car)| {car.write().unwrap().move_car_to_destination(&mut road_graph,  frame_time * 20.0, true);});   



/* 
        //roads.iter().for_each(|x| draw_dotted_line(x, &mut road_graph, false));
        
        nodes.for_each(|x| draw_node(x, true));
        
        
        


        //cars.iter().for_each(|car| println!("color: {:?}", car.get_color()));



        
            
        );
        
*/
        next_frame().await
    }
}



