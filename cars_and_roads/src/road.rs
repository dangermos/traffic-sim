pub type RoadID = i32;

pub struct Road {
    id: RoadID,
    from: RoadID,
    to: RoadID,
}

impl Road {
    pub fn new_road(from: RoadID, to: RoadID, id: RoadID) -> Self{
        Road {
            from,
            to,
            id,
        }
    }
}