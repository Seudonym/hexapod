use embedded_hal::i2c::I2c;

pub const DEFAULT_ADDRESS: u8 = 0x40;
pub const DEFAULT_FREQUENCY: u32 = 25_000_000;
pub const PRESCALE_MIN: u8 = 3;
pub const PRESCALE_MAX: u8 = 255;

/// PCA9685 I2c addresses
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
}

pub struct Mode1;
/// Mode1 register bits
impl Mode1 {
    pub const ALL_CALL: u8 = 0x01;
    pub const SUB1: u8 = 0x02;
    pub const SUB2: u8 = 0x04;
    pub const SUB3: u8 = 0x08;
    pub const SLEEP: u8 = 0x10;
    pub const AUTO_INC: u8 = 0x20;
    pub const EXT_CLK: u8 = 0x40;
    pub const RESTART: u8 = 0x80;
}

pub struct Mode2;
/// Mode2 register bits
impl Mode2 {
    pub const OUTNE0: u8 = 0x01;
    pub const OUTNE1: u8 = 0x02;
    pub const OUTDRV: u8 = 0x04;
    pub const OCH: u8 = 0x08;
    pub const INVRT: u8 = 0x10;
}

/// Output channel selection
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    C0 = 0,
    C1 = 1,
    C2 = 2,
    C3 = 3,
    C4 = 4,
    C5 = 5,
    C6 = 6,
    C7 = 7,
    C8 = 8,
    C9 = 9,
    C10 = 10,
    C11 = 11,
    C12 = 12,
    C13 = 13,
    C14 = 14,
    C15 = 15,
}

/// Device address configuration
pub struct Address(pub u8);

impl Address {
    /// Create a new address from the hardware configuration pins (A5, A4, A3, A2, A1, A0)
    pub fn new(a5: bool, a4: bool, a3: bool, a2: bool, a1: bool, a0: bool) -> Self {
        let mut addr = 0x40; // Base address
        if a0 {
            addr |= 0x01;
        }
        if a1 {
            addr |= 0x02;
        }
        if a2 {
            addr |= 0x04;
        }
        if a3 {
            addr |= 0x08;
        }
        if a4 {
            addr |= 0x10;
        }
        if a5 {
            addr |= 0x20;
        }
        Address(addr)
    }
}

impl Default for Address {
    fn default() -> Self {
        Address(DEFAULT_ADDRESS)
    }
}

/// PCA9685 I2C device
pub struct PCA9685<I2C> {
    /// I2C interface
    i2c: I2C,
    /// I2C address
    address: u8,
}

impl<I2C, E> PCA9685<I2C>
where
    I2C: I2c<Error = E>,
{
    /// Create a new PCA9685 instance
    pub fn new(i2c: I2C, address: Address) -> Result<Self, E> {
        let mut pca = PCA9685 {
            i2c,
            address: address.0,
        };

        pca.init()?;

        Ok(pca)
    }

    /// Reset the PCA9685
    pub fn reset(&mut self) -> Result<(), E> {
        // Reset the PCA9685 by writing 0x00 to the mode1 register
        self.write_register(Register::MODE1, Mode1::RESTART)?;
        Ok(())
    }

    /// Initialize the PCA9685
    fn init(&mut self) -> Result<(), E> {
        // Set the mode1 register to 0x00
        self.write_register(Register::MODE1, 0x01)?;
        Ok(())
    }

    /// Use low power mode
    pub fn sleep(&mut self) -> Result<(), E> {
        let mode1 = self.read_register(Register::MODE1)?;
        self.write_register(Register::MODE1, mode1 | Mode1::SLEEP)
    }

    /// Wake up from low power mode
    pub fn wake(&mut self) -> Result<(), E> {
        let mode1 = self.read_register(Register::MODE1)?;
        self.write_register(Register::MODE1, mode1 & !Mode1::SLEEP)
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

    /// Set the PWM for a channel
    pub fn set_pwm(&mut self, channel: Channel, on: u16, off: u16) -> Result<(), E> {
        let channel = match channel {
            Channel::C0 => Register::LED0_ON_L,
            Channel::C1 => Register::LED0_ON_L + 4,
            Channel::C2 => Register::LED0_ON_L + 8,
            Channel::C3 => Register::LED0_ON_L + 12,
            Channel::C4 => Register::LED0_ON_L + 16,
            Channel::C5 => Register::LED0_ON_L + 20,
            Channel::C6 => Register::LED0_ON_L + 24,
            Channel::C7 => Register::LED0_ON_L + 28,
            Channel::C8 => Register::LED0_ON_L + 32,
            Channel::C9 => Register::LED0_ON_L + 36,
            Channel::C10 => Register::LED0_ON_L + 40,
            Channel::C11 => Register::LED0_ON_L + 44,
            Channel::C12 => Register::LED0_ON_L + 48,
            Channel::C13 => Register::LED0_ON_L + 52,
            Channel::C14 => Register::LED0_ON_L + 56,
            Channel::C15 => Register::LED0_ON_L + 60,
        };
        self.write_register(channel, on as u8)?;
        self.write_register(channel + 1, (on >> 8) as u8)?;
        self.write_register(channel + 2, off as u8)?;
        self.write_register(channel + 3, (off >> 8) as u8)
    }

    /// Write a byte to a register
    pub fn write_register(&mut self, reg: u8, value: u8) -> Result<(), E> {
        let buf = [reg, value];
        self.i2c.write(self.address, &buf)
    }

    /// Read a byte from a register
    pub fn read_register(&mut self, reg: u8) -> Result<u8, E> {
        let mut buf = [0; 1];
        self.i2c.write_read(self.address, &[reg], &mut buf)?;
        Ok(buf[0])
    }
}
