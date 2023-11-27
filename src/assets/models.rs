pub struct Dynamics {
    pub position: (f32, f32),   //player position
    pub heading: f32,           //heading in rads, 0 ->  PI <-
    pub speed_vector: (f32, f32),
    pub thrust: f32,            //current thrust 0-100%
}

pub struct WeaponData {
    pub dynamics: Dynamics,
    pub power: u32,
    pub duration: u32
}

