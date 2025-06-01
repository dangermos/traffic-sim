
use rayon::prelude::*;
use macroquad::{prelude::*};
use cars_and_roads::road::{NodeID};
use cars_and_roads::level::Level;
use render::*;



#[macroquad::main("Main Render")]
async fn main() {

    
    //// INIT ////
    
    set_fullscreen(true);


    let level = Level::sim1();

    let road_graph = level.road_graph;
    let mut cars = level.cars;




    //// Game Loop ////
    loop { 

        draw_fps();
        
        road_graph.get_roads().iter().for_each(|x| draw_road(x));
        
        road_graph.get_nodes().iter().for_each(|x | draw_node(x, true));
        
        /* 

        for i in &mut cars {

            draw_car(&i, RED, true);

            i.move_car_on_road(get_frame_time() * 10.0, &road_graph);


            // println!("My Car ID is: {:?}\nI'm facing {} degrees!\nMy Current Position is {}\nI'm on road {:?}", i.get_id(), i.get_direction(), i.position, i.current_road);

        }
        */


        cars.get_cars_mut().par_iter_mut().for_each(|car| {car.move_car_to_destination(&road_graph, NodeID(4), get_frame_time() * 20.0, true);});
        
        
        cars.get_cars().iter().for_each(|x | draw_car(x, RED, true));
        


        next_frame().await
    }
}



