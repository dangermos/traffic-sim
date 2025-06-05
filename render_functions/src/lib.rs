use cars_and_roads::{draw_circle, draw_line, draw_text, draw_triangle, road::Node, Car, CarID, Color, Road, RoadGraph, Vec2, Vec4, BLUE, PINK, PURPLE, RED, WHITE};

pub fn draw_car(car: &Car, debug: bool) {
    let width = car.get_width();
    let height = car.get_height();

    let angle = car.get_direction() - std::f32::consts::FRAC_PI_2;
    let (r, g, b, a) = car.get_color();
    let color = Color::from_rgba(r, g, b, a);

    let body_len = height * 0.7;
    let roof_len = height * 0.3;
    let half_w = width / 2.0;

    let forward = Vec2::from_angle(angle);
    let right = Vec2::new(-forward.y, forward.x); // 90° perp

    let center = car.position;
    let front = center + forward * (body_len / 2.0);
    let rear = center - forward * (body_len / 2.0);
    let roof_front = center + forward * (body_len / 2.0 - roof_len);
    let roof_rear = center - forward * (body_len / 2.0 - roof_len);

    // Draw main body (larger rectangle)
    let body_corners = [
        rear - right * half_w,
        rear + right * half_w,
        front + right * half_w,
        front - right * half_w,
    ];

    draw_triangle(body_corners[0], body_corners[1], body_corners[2], color);
    draw_triangle(body_corners[2], body_corners[3], body_corners[0], color);

    // Draw windshield / roof (smaller polygon)
    let roof_corners = [
        roof_rear - right * (half_w * 0.6),
        roof_rear + right * (half_w * 0.6),
        roof_front + right * (half_w * 0.4),
        roof_front - right * (half_w * 0.4),
    ];

    let roof_color = Color::from_rgba(200, 200, 200, 200); // light grey roof
    draw_triangle(roof_corners[0], roof_corners[1], roof_corners[2], roof_color);
    draw_triangle(roof_corners[2], roof_corners[3], roof_corners[0], roof_color);

    if debug {
        // Heading arrow
        let dir = forward.normalize();
        let tip = center + dir * 20.0;
        let base = center + dir * 5.0;
        let perp = Vec2::new(-dir.y, dir.x) * 4.0;
        draw_line(center.x, center.y, tip.x, tip.y, 2.0, color);
        draw_triangle(tip, base + perp, base - perp, color);

        // Car ID
        draw_text(&format!("{:?}", car.get_id()), center.x, center.y - 10.0, 16.0, color);
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


