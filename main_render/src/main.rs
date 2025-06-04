
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

     let level = Level::sim_roundabout();

    //let level = Level::sim1();

    let mut road_graph = level.road_graph;
    let mut cars: Vec<Car> = road_graph.get_cars().to_vec();




    //// Game Loop ////
    loop { 

        draw_fps();
        
        draw_roads(&mut road_graph, true);

        let roads = road_graph.get_roads().to_vec();
        roads.iter().for_each(|x| draw_dotted_line(x, &mut road_graph, false));
        
        road_graph.get_nodes().iter().for_each(|x| draw_node(x, true));
        
        
        
        let frame_time = get_frame_time();


        cars.iter().for_each(|car| println!("color: {:?}", car.get_color()));

        //cars.par_iter_mut().for_each(|car| {car.move_car_to_destination(&mut road_graph, car.destination, frame_time * 20.0, true);});   
        cars.iter_mut().for_each(|car| {car.move_car_to_destination(&mut road_graph, car.destination, frame_time * 20.0, true);});


        
        cars.iter().for_each(
            |x | {
                draw_car(x, true);
                println!("path: {:?}", x.get_path());
            }
            
        );
        

        next_frame().await
    }
}



