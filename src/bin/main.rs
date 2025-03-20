#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::{delay, main};
use log::info;

use esp_hal::i2c;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    info!("panic");
    loop {}
}

extern crate alloc;

#[main]
fn main() -> ! {
    // generator version: 0.3.1
    esp_println::logger::init_logger_from_env();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let mut i2c = i2c::master::I2c::new(peripherals.I2C0, i2c::master::Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    let pca9685_addr = 0x40;
    let servo_min = 120;
    let servo_max = 520;
    let num_servos = 2;

    let result = i2c.write(pca9685_addr, &[0x00, 0x90]).unwrap(); // Use low power mode to change PWM frequency, set restart
    info!("Low power mode status: {:?}", result);
    let result = i2c.write(pca9685_addr, &[0xFE, 0x79]).unwrap(); // Set PWM frequency to 50 Hz
    info!("PWM freq change status: {:?}", result);
    let result = i2c.write(pca9685_addr, &[0x00, 0x20]).unwrap(); // Use normal mode
    info!("Normal mode status: {:?}", result);

    let mut angle = 0;
    let delay = delay::Delay::new();
    loop {
        if angle > 180 {
            angle = 0;
        }

        let pulse = servo_min + (angle * (servo_max - servo_min) / 180);
        info!("Angle: {}, Pulse: {}", angle, pulse);

        for address in 0x06..0x06 + num_servos * 4 {
            let _result = i2c.write(
                pca9685_addr,
                &[
                    address,
                    0x00,
                    0x00,
                    (pulse & 0xFF) as u8,
                    ((pulse >> 8) & 0xFF) as u8,
                ],
            );
        }

        angle = angle + 45;
        delay.delay_millis(2000);
    }
}
