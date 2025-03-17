#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::main;
use esp_hal::time::{Duration, Instant};
use esp_hal::timer::timg::TimerGroup;
use log::info;

use esp_hal::i2c;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
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

    //// Dont need wifi yet
    //let timg0 = TimerGroup::new(peripherals.TIMG0);
    //let _init = esp_wifi::init(
    //    timg0.timer0,
    //    esp_hal::rng::Rng::new(peripherals.RNG),
    //    peripherals.RADIO_CLK,
    //)
    //.unwrap();

    let mut i2c = i2c::master::I2c::new(peripherals.I2C0, i2c::master::Config::default())
        .unwrap()
        .with_sda(peripherals.GPIO21)
        .with_scl(peripherals.GPIO22);

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}

        // Check which I2C devices are connected
        for address in 0..128 {
            match i2c.read(address, &mut [0]) {
                Ok(_) => info!("Found I2C device at address: 0x{:02X}", address),
                Err(_) => (),
            }
        }
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}
