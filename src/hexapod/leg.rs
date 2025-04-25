use crate::pca9685::Channel;

pub struct Leg {
    pub coxa_length: f64,
    pub femur_length: f64,
    pub tibia_length: f64,

    pub coxa_angle: f64,
    pub femur_angle: f64,
    pub tibia_angle: f64,

    pub coxa_channel: Channel,
    pub femur_channel: Channel,
    pub tibia_channel: Channel,
}

impl Leg {
    pub fn new(coxa_channel: Channel, femur_channel: Channel, tibia_channel: Channel) -> Self {
        Leg {
            coxa_length: 0.0,
            femur_length: 0.0,
            tibia_length: 0.0,

            coxa_angle: 0.0,
            femur_angle: 0.0,
            tibia_angle: 0.0,

            coxa_channel,
            femur_channel,
            tibia_channel,
        }
    }
}
