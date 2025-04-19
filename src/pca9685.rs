use bitflags::bitflags;
use embedded_hal::{delay::DelayNs, i2c::I2c};

pub const DEFAULT_ADDRESS: u8 = 0x40;
pub const DEFAULT_FREQUENCY: u32 = 25_000_000;
pub const PRESCALE_MIN: u8 = 3;
pub const PRESCALE_MAX: u8 = 255;

/// PCA9685 I2c addresses
pub enum Register {
    Mode1 = 0x00,
    Mode2 = 0x01,
    SubAddr1 = 0x02,
    SubAddr2 = 0x03,
    SubAddr3 = 0x04,
    AllCallAddr = 0x05,

    LED0OnL = 0x06,
    LED0OnH = 0x07,
    LED0OffL = 0x08,
    LED0OffH = 0x09,

    // ...
    AllLEDOnL = 0xFA,
    AllLEDOnH = 0xFB,
    AllLEDOffL = 0xFC,
    AllLEDOffH = 0xFD,

    PreScale = 0xFE,
    TestMode = 0xFF,
}

bitflags! {
    /// Mode1 register bits
    pub struct Mode1: u8 {
        const ALL_CALL = 0x01;
        const SUB1 = 0x02;
        const SUB2 = 0x04;
        const SUB3 = 0x08;
        const SLEEP = 0x10;
        const AUTO_INC = 0x20;
        const EXT_CLK = 0x40;
        const RESTART = 0x80;
    }
}

bitflags! {
/// Mode2 register bits
    pub struct Mode2: u8 {
        const OUTNE0 = 0x01;
        const OUTNE1 = 0x02;
        const OUTDRV = 0x04;
        const OCH = 0x08;
        const INVRT = 0x10;
    }
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

    /// Returns the default address (0x40)
    pub fn default() -> Self {
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
    pub fn reset<D>(&mut self, delay: &mut D) -> Result<(), E>
    where
        D: DelayNs,
    {
        // Reset the PCA9685 by writing 0x00 to the mode1 register
        self.write_register(Register::Mode1, Mode1::RESTART.bits())?;
        delay.delay_ms(5);
        Ok(())
    }

    /// Initialize the PCA9685
    fn init(&mut self) -> Result<(), E> {
        // Set the mode1 register to 0x00
        self.write_register(Register::Mode1, 0x01)?;
        Ok(())
    }

    /// Write a byte to a register
    pub fn write_register(&mut self, reg: Register, value: u8) -> Result<(), E> {
        let mut buf = [reg as u8, value];
        self.i2c.write(self.address, &buf)
    }

    /// Read a byte from a register
    pub fn read_register(&mut self, reg: Register) -> Result<u8, E> {
        let mut buf = [0; 1];
        self.i2c.write_read(self.address, &[reg as u8], &mut buf)?;
        Ok(buf[0])
    }
}
