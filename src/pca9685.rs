use embedded_hal::i2c;

/// PCA9685 driver
#[derive(Debug)]
pub struct PCA9685<I2C> {
    /// I2C implementation
    i2c: I2C,
    /// I2C device address
    address: DeviceAddr,
}

pub struct Register;
impl Register {
    pub const MODE1: u8 = 0x00;
    pub const MODE2: u8 = 0x01;
    pub const SUBADR1: u8 = 0x02;
    pub const SUBADR2: u8 = 0x03;
    pub const SUBADR3: u8 = 0x04;
    pub const ALLCALLADR: u8 = 0x05;

    pub const LED0_ON_L: u8 = 0x06;
    pub const LED0_ON_H: u8 = 0x07;
    pub const LED0_OFF_L: u8 = 0x08;
    pub const LED0_OFF_H: u8 = 0x09;

    pub const ALL_LED_ON_L: u8 = 0xFA;
    pub const ALL_LED_ON_H: u8 = 0xFB;
    pub const ALL_LED_OFF_L: u8 = 0xFC;
    pub const ALL_LED_OFF_H: u8 = 0xFD;

    pub const PRESCALE: u8 = 0xFE;
    pub const TEST_MODE: u8 = 0xFF;

    // MODE1 bits
    pub const MODE1_RESTART: u8 = 0x80;
    pub const MODE1_EXTCLK: u8 = 0x40;
    pub const MODE1_AI: u8 = 0x20;
    pub const MODE1_SLEEP: u8 = 0x10;
    pub const MODE1_SUB1: u8 = 0x08;
    pub const MODE1_SUB2: u8 = 0x04;
    pub const MODE1_SUB3: u8 = 0x02;
    pub const MODE1_ALLCALL: u8 = 0x01;

    // MODE2 bits
    pub const MODE2_INVRT: u8 = 0x10;
    pub const MODE2_OCH: u8 = 0x08;
    pub const MODE2_OUTDRV: u8 = 0x04;
    pub const MODE2_OUTNE1: u8 = 0x02;
    pub const MODE2_OUTNE0: u8 = 0x01;

    pub const PCA9685_ADDRESS: u8 = 0x40;
    pub const FREQUENCY_OSCILLATOR: u32 = 25_000_000;

    pub const PRESCALE_MIN: u8 = 0x03;
    pub const PRESCALE_MAX: u8 = 0xFF;
}

pub enum Channel {
    Channel0,
    Channel1,
    Channel2,
    Channel3,
    Channel4,
    Channel5,
    Channel6,
    Channel7,
    Channel8,
    Channel9,
    Channel10,
    Channel11,
    Channel12,
    Channel13,
    Channel14,
    Channel15,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceAddr {
    /// 0x40
    Addr0 = 0x40,
    /// 0x70
    AddrAll = 0x70,
}

impl<I2C, E> PCA9685<I2C>
where
    I2C: i2c::ErrorType<Error = E> + i2c::I2c<Error = E>,
{
    /// Create a new PCA9685 driver from an I2C peripheral and the device address
    pub fn new(i2c: I2C, address: DeviceAddr) -> Result<Self, E> {
        Ok(Self { i2c, address })
    }

    /// Write a byte to a register
    pub fn write_register(&mut self, register: u8, data: u8) -> Result<(), E> {
        self.i2c.write(self.address as u8, &[register, data])
    }

    /// Read a byte from a register
    pub fn read_register(&mut self, register: u8) -> Result<u8, E> {
        let mut data = [0];
        self.i2c
            .write_read(self.address as u8, &[register], &mut data)?;
        Ok(data[0])
    }

    /// Use low power mode
    pub fn sleep(&mut self) -> Result<(), E> {
        let mode1 = self.read_register(Register::MODE1)?;
        self.write_register(Register::MODE1, mode1 | Register::MODE1_SLEEP)
    }

    /// Wake up from low power mode
    pub fn wake(&mut self) -> Result<(), E> {
        let mode1 = self.read_register(Register::MODE1)?;
        self.write_register(Register::MODE1, mode1 & !Register::MODE1_SLEEP)
    }

    /// Set the PWM frequency
    /// The formula for prescale is given by:
    ///      prescale = round(osc_clock / (4096 * frequency)) - 1
    pub fn set_pwm_frequency(&mut self, frequency: f32) -> Result<(), E> {
        let prescale = (25_000_000.0 / (4096.0 * frequency) - 1.0).round() as u8;
        self.sleep()?;
        self.write_register(Register::PRESCALE, prescale)?;
        self.wake()
    }

    /// Set PWM
    pub fn set_pwm(&mut self, channel: Channel, on: u16, off: u16) -> Result<(), E> {
        let channel = match channel {
            Channel::Channel0 => Register::LED0_ON_L,
            Channel::Channel1 => Register::LED0_ON_L + 4,
            Channel::Channel2 => Register::LED0_ON_L + 8,
            Channel::Channel3 => Register::LED0_ON_L + 12,
            Channel::Channel4 => Register::LED0_ON_L + 16,
            Channel::Channel5 => Register::LED0_ON_L + 20,
            Channel::Channel6 => Register::LED0_ON_L + 24,
            Channel::Channel7 => Register::LED0_ON_L + 28,
            Channel::Channel8 => Register::LED0_ON_L + 32,
            Channel::Channel9 => Register::LED0_ON_L + 36,
            Channel::Channel10 => Register::LED0_ON_L + 40,
            Channel::Channel11 => Register::LED0_ON_L + 44,
            Channel::Channel12 => Register::LED0_ON_L + 48,
            Channel::Channel13 => Register::LED0_ON_L + 52,
            Channel::Channel14 => Register::LED0_ON_L + 56,
            Channel::Channel15 => Register::LED0_ON_L + 60,
        };
        self.write_register(channel, on as u8)?;
        self.write_register(channel + 1, (on >> 8) as u8)?;
        self.write_register(channel + 2, off as u8)?;
        self.write_register(channel + 3, (off >> 8) as u8)
    }
}
