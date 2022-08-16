fn main() {
    println!("Hello, world!");
}

struct Elevator {
    floor: Floor,
    direction: Option<Direction>,
    pickup_requests: Vec<PickupRequest>,
    dropoff_requests: Vec<DropoffRequest>,
}

impl Elevator {
    fn new() -> Self {
        Elevator {
            floor: 0,
            direction: None,
            pickup_requests: vec![],
            dropoff_requests: vec![],
        }
    }
}

impl Default for Elevator {
    fn default() -> Self {
        Elevator::new()
    }
}

struct PickupRequest {
    floor: Floor,
    direction: Direction,
}

struct DropoffRequest {
    floor: Floor,
}

enum Direction {
    Up,
    Down,
}

type Floor = i32;
