#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::main;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::gpio::{
        Io,
        Level,
        Output,
        Pull,};
use esp_hal::peripheral::{Peripheral};
use esp_hal::rtc_cntl::Rtc;
use esp_hal::spi::{master::{Config, Spi}, DataMode, Mode};
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;
use esp_hal::i2c::master::{I2c, Config as i2cConfig};

use log::info;

use embedded_hal_bus::spi::ExclusiveDevice;

use embedded_graphics::{
    pixelcolor::{Rgb565, BinaryColor},
    prelude::*,
    primitives::{Circle, Primitive, PrimitiveStyle, Triangle},
    text::{Alignment, Text},
    mono_font::{ascii::FONT_6X10, ascii::FONT_10X20, MonoTextStyle},
};

use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, models::ST7789};
use mipidsi::options::{ColorInversion, Orientation, Rotation};


use fugit::RateExtU32;
use fugit::HertzU32;

use embedded_graphics::draw_target::*;
use embedded_graphics::geometry::*;
use embedded_graphics::pixelcolor::*;
use embedded_graphics::primitives::*;
use embedded_graphics::Drawable;

extern crate alloc;


#[main]
fn main() -> ! {

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::_160MHz);
    let peripherals = esp_hal::init(config);

    let mut delay = Delay::new();

    let lcb_en = Output::new(peripherals.GPIO42, Level::High);
    let lcb_en = Output::new(peripherals.GPIO10, Level::High);

    let dc = Output::new(peripherals.GPIO11, Level::Low);

    let sck = Output::new(peripherals.GPIO40,  Level::Low);
    let miso = Output::new(peripherals.GPIO38,  Level::Low);
    let mosi = Output::new(peripherals.GPIO41,  Level::Low);
    let cs = Output::new(peripherals.GPIO12,  Level::Low);

    let spi = Spi::new(peripherals.SPI2,Config::default(),).unwrap().with_miso(miso).with_mosi(mosi).with_sck(sck);
    let config = Config::default().with_frequency(62500.kHz()).with_mode(Mode::_0); 

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, delay).unwrap();

    let mut buffer = [0_u8; 512];
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    let mut display = Builder::new(ST7789, di).display_size(240, 320).orientation(Orientation::new().rotate(Rotation::Deg90)).invert_colors(ColorInversion::Inverted).init(&mut delay).unwrap();

    display.clear(Rgb565::BLACK).unwrap();

    /*
    let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::BLACK)
        .build();
*/

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let delay = Delay::new();


    //let mut i2c = I2c::new(peripherals.I2C0, i2cConfig::default(),).unwrap().with_sda(peripherals.GPIO18).with_scl(peripherals.GPIO8);
    let mut i2c = I2c::new(peripherals.I2C0, {
        let mut i2c_config = i2cConfig::default();
        i2c_config.frequency = 100.kHz();
        i2c_config.timeout;
        i2c_config
    },).unwrap().with_sda(peripherals.GPIO18).with_scl(peripherals.GPIO8);

     
    let mut DeltaY=0;
    let mut DeltaX=0;

    loop {
        /*
        let mut data = [0u8];
        i2c.write_read(0x55, &[0xaa], &mut data).ok(); //0x55 is T-keyboard I2C ADDRESS !
        info!("data: {:#?}", data);

        let step = 20;
        if (data == [104u8]){
            DeltaX +=step;
        }
        if (data == [102u8]){
            DeltaX -=step;
        }
        if (data == [118u8]){
            DeltaY +=step;
        }
        if (data == [116u8]){
            DeltaY -=step;
        }*/

        let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
         // Draw centered text.
        let text = "Alright";

        display.clear(Rgb565::BLACK);
        
        /*let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
            .fill_color(Rgb565::BLACK)
            .build();*/

        Text::with_alignment(
            text,
            display.bounding_box().center() + Point::new(0+DeltaX, 15+DeltaY),
            character_style,
            Alignment::Center,
        )
        .draw(&mut display);

        //info!("Hello world!");
        //delay.delay_millis(100);
        
        //delay.delay_millis(500);
/*
        let character_style = MonoTextStyle::new(&FONT_10X20, Rgb565::WHITE);
         // Draw centered text.
        let text = "Alright Brother :D";
        Text::with_alignment(
            text,
            display.bounding_box().center() + Point::new(20, 20),
            character_style,
            Alignment::Center,
        )
        .draw(&mut display);
        */
        //delay.delay_millis(500);
        //display.clear(Rgb565::BLACK).unwrap();
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}
