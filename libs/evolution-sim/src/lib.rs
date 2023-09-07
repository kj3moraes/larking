
pub struct Simulation {
    world: World,
    age: usize
}

pub struct World {
    pub animals: Vec<Animal>,
    pub food: Vec<Food>,
}

pub struct Animal {
    pub position: Position,
    pub energy: f64,
    pub direction: f64,
    pub speed: f64,
    pub size: f64,
    pub color: Color,
}

pub struct Food {
    pub position: Position,
    pub energy: f64,
    pub size: f64,
    pub color: Color,
}

pub Position {
    x: f32,
    y: f32,
}