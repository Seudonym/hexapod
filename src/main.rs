use esp_idf_svc::hal::{
    delay::FreeRtos,
    i2c::{I2cConfig, I2cDriver},
    prelude::Peripherals,
};

use hexapod_v2::pca9685::{self, Channel, DeviceAddr, Register};

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let i2c_config = I2cConfig::new();
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;
    let i2c_driver = match I2cDriver::new(peripherals.i2c0, sda, scl, &i2c_config) {
        Ok(driver) => driver,
        Err(e) => {
            log::error!("Failed to initialize I2C driver: {:?}", e);
            loop {}
        }
    };

    let mut pca9685 = match pca9685::PCA9685::new(i2c_driver, DeviceAddr::Addr0) {
        Ok(pca9685) => pca9685,
        Err(e) => {
            log::error!("Failed to initialize PCA9685: {:?}", e);
            loop {}
        }
    };

    println!("PCA9685 initialized");

    const SERVO_MIN: u16 = 135;
    const SERVO_MAX: u16 = 530;

    pca9685.set_pwm_frequency(50.0).unwrap();

    let mut pulse = 0;
    loop {
        pulse = pulse + 1;
        if pulse == SERVO_MAX {
            FreeRtos::delay_ms(1000);
            pulse = SERVO_MIN;
            pca9685.set_pwm(Channel::Channel0, 0, pulse).unwrap();
            pca9685.set_pwm(Channel::Channel1, 0, pulse).unwrap();
            FreeRtos::delay_ms(1000);
        }
        pca9685.set_pwm(Channel::Channel0, 0, pulse).unwrap();
        pca9685.set_pwm(Channel::Channel1, 0, pulse).unwrap();
        FreeRtos::delay_ms(10);
    }
}
