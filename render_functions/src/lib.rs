use cars_and_roads::{draw_circle, draw_line, draw_text, draw_triangle, road::Node, Car, Color, Road, Vec2, BLUE, PURPLE, RED, WHITE, PINK};

pub fn draw_car(car: &Car, color: Color, debug: bool) {

    let width = car.get_width();
    let height = car.get_height();

    let angle_rad = car.get_direction() - std::f32::consts::FRAC_PI_2;
    let half_w = width / 2.0;
    let half_h = height / 2.0;

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
        draw_line(start.x, start.y, tip.x, tip.y, 2.0, PURPLE);
    
        // Compute arrowhead triangle base corners
        let perp = Vec2::new(-direction.y, direction.x); // 90° rotated vector
        let base = tip - direction * (tip_size + 2.0);   // back off from tip a bit
        let left = base + perp * tip_size;
        let right = base - perp * tip_size;
    
        draw_triangle(tip, left, right, PURPLE);
    }
    
    

    

}

pub fn draw_road(road: &Road) {

    let color = if road.one_way {PINK} else {WHITE};


    for pair in road.points.windows(2) {
        draw_line(pair[0].x, pair[0].y, pair[1].x, pair[1].y, 4.0, color);
    }
}

pub fn draw_node(node: &Node, debug: bool) {
    draw_circle(node.position.x, node.position.y, 2.0, RED);
    if debug {
        draw_text(&node.id.to_string(), node.position.x + 50.0, node.position.y + 10.0, 32.0, BLUE);
    }
}


