const GRAVITY: f32 = 9.81;
#[derive(Debug)]
pub enum GravityOnPlanets {
    MERCURY(f32),
    MOON(f32),
    VENUS(f32),
    MARS(f32),
    JUPYTER(f32),
    NEPTUNE(f32),
    URANUS(f32),
    SATURN(f32),
}

pub struct Planet {
    id: i32,
    name: String,
    gravity: f32,
}

impl Planet {
    pub fn new(name: String, planet_gravity: f32, id: i32) -> Planet {
        Planet {
            id,
            gravity: planet_gravity,
            name,
        }
    }
    pub fn cal_gravity_on_planet(&self, weight: f32) -> f32 {
        return (weight / GRAVITY) * self.gravity;
    }

    pub fn get_name(&self) -> &String {
        let string = &self.name;
        string
    }
}
