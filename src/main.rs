use esp_idf_svc::hal;
use esp_idf_svc::hal::prelude::Peripherals;
use hexapod::pca9685;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");

    let peripherals = Peripherals::take().unwrap();
    let sda = peripherals.pins.gpio21;
    let scl = peripherals.pins.gpio22;

    let mut delay = hal::delay::Delay::new_default();
    let i2c =
        hal::i2c::I2cDriver::new(peripherals.i2c0, sda, scl, &hal::i2c::config::Config::new())
            .unwrap();

    let mut pca = pca9685::PCA9685::new(i2c, pca9685::Address::default()).unwrap();

    let _ = pca.reset(&mut delay).unwrap();
}
