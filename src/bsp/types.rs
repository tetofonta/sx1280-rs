use rp2040_hal::gpio::{Pin, FunctionSpi, FunctionSioOutput, FunctionSioInput, PullDown, PullUp, bank0::*};
use rp2040_hal::pac::{SPI0, SPI1};
use rp2040_hal::Spi;
use rp2040_hal::spi::{Enabled};

///                   +-------------------+
///                   VBUS              GP0
///                   VSYS              GP1
///                   GND               GND
///                   3V3_EN            GP2         - RST   +
///                   3V3               GP3         - BUSY  |
///                   VREF              GP4         - MISO  |
///                   GP28              GP5         - CS    |
///                   AGND              GND                 +--- TRX 0
///                   GP27              GP6         - SCK   |
///                   GP26              GP7         - MOSI  +
///                   RUN               GP8
///                   GP22              GP9
///                   GND               GND
///                   GP21             GP10         - SCK    +
///                   GP20             GP11         - MOSI   |
///                   GP19             GP12         - MISO   |
///                   GP18             GP13         - CS     |
///                   GND               GND                  +--- TRX 1
///                   GP17             GP14         - RST    |
///                   GP16             GP15         - BUSY   +
///                   +-------------------+
///

pub type Spi0Miso = Pin<Gpio4, FunctionSpi, PullDown>;
pub type Spi0Mosi = Pin<Gpio7, FunctionSpi, PullDown>;
pub type Spi0Sck = Pin<Gpio6, FunctionSpi, PullDown>;
pub struct Spi0Pins {
    pub mosi: Spi0Mosi,
    pub miso: Spi0Miso,
    pub sck: Spi0Sck
}
pub type Spi0 = Spi<Enabled, SPI0, (Spi0Mosi, Spi0Miso, Spi0Sck)>;
impl Spi0Pins {
    pub fn to_pins(self) -> (Spi0Mosi, Spi0Miso, Spi0Sck) {
        (self.mosi, self.miso, self.sck)
    }
}



pub type Spi1Miso = Pin<Gpio12, FunctionSpi, PullDown>;
pub type Spi1Mosi = Pin<Gpio11, FunctionSpi, PullDown>;
pub type Spi1Sck = Pin<Gpio10, FunctionSpi, PullDown>;
pub struct Spi1Pins {
    pub mosi: Spi1Mosi,
    pub miso: Spi1Miso,
    pub sck: Spi1Sck
}
pub type Spi1 = Spi<Enabled, SPI1, (Spi1Mosi, Spi1Miso, Spi1Sck)>;
impl Spi1Pins {
    pub fn to_pins(self) -> (Spi1Mosi, Spi1Miso, Spi1Sck) {
        (self.mosi, self.miso, self.sck)
    }
}


pub type PinResetA = Pin<Gpio2, FunctionSioOutput, PullDown>;
pub type PinCSA = Pin<Gpio5, FunctionSioOutput, PullDown>;
pub type PinBusyA = Pin<Gpio3, FunctionSioInput, PullUp>;


pub type PinResetB = Pin<Gpio14, FunctionSioOutput, PullDown>;
pub type PinCSB = Pin<Gpio13, FunctionSioOutput, PullDown>;
pub type PinBusyB = Pin<Gpio15, FunctionSioInput, PullUp>;
