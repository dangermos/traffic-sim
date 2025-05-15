use cars_and_roads::{draw_triangle, draw_line, Car, Color, Road, Vec2};

pub fn draw_car(car: &Car, color: Color) {

    let width = car.get_width();
    let height = car.get_height();

    let angle_rad = car.get_direction().to_radians();
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



    

}

pub fn draw_road(road: &Road, color: Color) {
    for pair in road.points.windows(2) {
        draw_line(pair[0].x, pair[0].y, pair[1].x, pair[1].y, 4.0, color);
    }
}


