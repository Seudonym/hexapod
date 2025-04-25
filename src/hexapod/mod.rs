const SERVO_MIN: u16 = 120;
const SERVO_MAX: u16 = 530;

pub mod leg;

use crate::pca9685::{Address, Channel, PCA9685};
use embedded_hal::i2c::I2c;
use leg::Leg;

pub struct Hexapod<I2C, E>
where
    I2C: I2c<Error = E>,
{
    legs: Vec<Leg>,
    servo_controller: PCA9685<I2C>,
}

impl<I2C, E> Hexapod<I2C, E>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: Address) -> Result<Self, E> {
        let mut servo_controller = PCA9685::new(i2c, address)?;
        servo_controller.reset()?;
        servo_controller.set_pwm_frequency(50.0)?;

        let legs = vec![Leg::new(Channel::C0, Channel::C1, Channel::C2)];

        Ok(Hexapod {
            legs,
            servo_controller,
        })
    }

    pub fn update_leg_angles(
        &mut self,
        leg_index: usize,
        coxa_angle: f64,
        femur_angle: f64,
        tibia_angle: f64,
    ) -> Result<(), E> {
        if let Some(leg) = self.legs.get_mut(leg_index) {
            leg.coxa_angle = coxa_angle;
            leg.femur_angle = femur_angle;
            leg.tibia_angle = tibia_angle;

            // Convert angles to PWM values
            let coxa_pwm = Self::angle_to_pwm(coxa_angle);
            let femur_pwm = Self::angle_to_pwm(femur_angle);
            let tibia_pwm = Self::angle_to_pwm(tibia_angle);

            // Update the servo controller with the new angles
            self.servo_controller
                .set_pwm(leg.coxa_channel, 0, coxa_pwm)?;
            self.servo_controller
                .set_pwm(leg.femur_channel, 0, femur_pwm)?;
            self.servo_controller
                .set_pwm(leg.tibia_channel, 0, tibia_pwm)?;
        }

        Ok(())
    }

    fn angle_to_pwm(angle: f64) -> u16 {
        // Convert angle to PWM value
        (angle / 180.0 * (SERVO_MAX - SERVO_MIN) as f64 + SERVO_MIN as f64) as u16
    }
}
