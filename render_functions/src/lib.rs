use cars_and_roads::{draw_circle, draw_line, draw_text, draw_triangle, road::Node, Car, CarID, Color, Road, RoadGraph, Vec2, Vec4, BLUE, PINK, PURPLE, RED, WHITE};

pub fn draw_car(car: &Car, debug: bool) {

    let width = car.get_width();
    let height = car.get_height();

    let angle_rad = car.get_direction() - std::f32::consts::FRAC_PI_2;
    let half_w = width / 2.0;
    let half_h = height / 2.0;

    let (r,g,b,a) = car.get_color();


    let color = Color::from_rgba(r,g,b,a);
    



    // Define the rectangle corners relative to center
    let corners = [
        Vec2::new(-half_w, -half_h), // top-left
        Vec2::new( half_w, -half_h), // top-right
        Vec2::new( half_w,  half_h), // bottom-right
        Vec2::new(-half_w,  half_h), // bottom-left
    ];

    // Rotate and translate each corner
    let rotated: Vec<Vec2> = corners.iter().map(|p| {
        let rotated_x = p.x * angle_rad.cos() - p.y * angle_rad.sin();
        let rotated_y = p.x * angle_rad.sin() + p.y * angle_rad.cos();
        car.position + Vec2::new(rotated_x, rotated_y)
    }).collect();

    // Draw it as two triangles
    draw_triangle(rotated[0], rotated[1], rotated[2], color);
    draw_triangle(rotated[2], rotated[3], rotated[0], color);

    if debug {
        let direction = Vec2::from_angle(car.get_direction()).normalize();
        let speed = car.velocity.max(1.0); // prevent scaling to zero
    
        let arrow_length = 15.0 + speed * 1.5; // total arrow length
        let tip_size = 4.0 + speed * 0.4;      // size of the arrowhead
    
        let start = car.position;
        let tip = start + direction * arrow_length;
    
        // Draw the main arrow shaft
        draw_line(start.x, start.y, tip.x, tip.y, 2.0, color);
    
        // Compute arrowhead triangle base corners
        let perp = Vec2::new(-direction.y, direction.x); // 90° rotated vector
        let base = tip - direction * (tip_size + 2.0);   // back off from tip a bit
        let left = base + perp * tip_size;
        let right = base - perp * tip_size;
    
        draw_triangle(tip, left, right, color);

        let id = format!("{:?}", car.get_id());
        draw_text(&id, car.position.x, car.position.y - 10.0, 18.0, color);

    }
    

}

pub fn mix_colors(colors: Vec<(u8, u8, u8, u8)>) -> Option<(u8, u8, u8, u8)> {

    let len = colors.len() as u32;

    if colors.is_empty() {
        return None;
    }

    let mut sum_r: u32 = 0;
    let mut sum_g: u32 = 0;
    let mut sum_b: u32 = 0;
    let mut sum_a: u32 = 0;

    for (r, g, b, a) in colors {
        sum_r += r as u32;
        sum_g += g as u32;
        sum_b += b as u32;
        sum_a += a as u32;
    }

    let avg_r = (sum_r / len) as u8;
    let avg_g = (sum_g / len) as u8;
    let avg_b = (sum_b / len) as u8;
    let avg_a = (sum_a / len) as u8;

    Some((avg_r, avg_g, avg_b, avg_a))
}

pub fn draw_roads(road_graph: &mut RoadGraph, debug: bool) -> () {

    for (_id, road) in road_graph.get_roads() {

        let road = road.read().unwrap();

        let color = if road.one_way {PINK} else {WHITE};

        for pair in road.points.windows(2) {

            let (x1, y1, x2, y2) = (pair[0].x, pair[0].y, pair[1].x, pair[1].y);
            draw_line(x1, y1, x2, y2, 4.0, color);
            if debug {
                let text = format!("Cars {:?} are on this Road", road_graph.get_cars().iter().map(|(id, _car)| id).collect::<Vec<_>>());
                draw_text(&text,  (x1 + x2) / 2.0, ((y1 + y2) / 2.0) - 100.0, 14.0, color);
            }
        }
        
    }
}

pub fn draw_dotted_line(road: &Road, road_graph: &mut RoadGraph, debug: bool) {
    let segment_length = 10.0;
    let spacing = 5.0;

    // Mix all car‐colors once
    let color = mix_colors(
        road_graph
            .get_cars()
            .iter()
            .map(|(_id, car)| car.read().unwrap().get_color())
            .collect(),
    );
    let (r, g, b, a) = color.unwrap_or_default();

    let points = &road.points;

    // Iterate over each consecutive pair in `road.points`
    for window in points.windows(2) {
        let from = window[0];
        let to = window[1];

        let direction = (to - from).normalize();
        let distance = from.distance(to);
        let mut pos = from;

        let step = direction * (segment_length + spacing);
        let num_dots = (distance / (segment_length + spacing)).floor() as usize;

        for _ in 0..num_dots {
            let end = pos + direction * segment_length;
            draw_line(
                pos.x,
                pos.y,
                end.x,
                end.y,
                5.0,
                Color::from_rgba(r, g, b, a),
            );
            pos += step;
        }
    }
}

pub fn draw_node(node: &Node, debug: bool) {
    draw_circle(node.position.x, node.position.y, 2.0, RED);
    if debug {
        draw_text(&node.id.to_string(), node.position.x + 50.0, node.position.y + 10.0, 32.0, BLUE);
    }
}


