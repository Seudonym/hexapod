use esp_idf_svc::hal;
use esp_idf_svc::hal::prelude::Peripherals;
use hexapod::hexapod::Hexapod;
pub(crate) use hexapod::pca9685::Address;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let delay = hal::delay::Delay::new_default();
    let i2c =
        hal::i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &hal::i2c::config::Config::new())
            .unwrap();

    let mut hexapod = Hexapod::new(i2c, Address::default()).unwrap();
    let mut angle = 0.0;
    loop {
        hexapod.update_leg_angles(0, angle, angle, angle).unwrap();
        delay.delay_ms(2000);
        angle += 10.0;

        if angle > 180.0 {
            angle = 0.0;
        }
    }
}
