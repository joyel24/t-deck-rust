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
use esp_hal::peripheral::{Peripheral}; // needed for `into_ref`
//use esp_hal::prelude::*;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::spi::{master::{Config, Spi}, DataMode, Mode};
use embedded_hal::digital::OutputPin;
use embedded_hal::digital::StatefulOutputPin;

use log::info;

use embedded_hal_bus::spi::ExclusiveDevice;

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, Primitive, PrimitiveStyle, Triangle},
};

use mipidsi::interface::SpiInterface;
use mipidsi::{Builder, models::ST7789};
use mipidsi::options::ColorInversion;


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
    // generator version: 0.2.2

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut delay = Delay::new();

    let lcb_en = Output::new(peripherals.GPIO42, Level::High);//  Level::Low);
    let lcb_en = Output::new(peripherals.GPIO10, Level::High);//  Level::Low);

    let dc = Output::new(peripherals.GPIO11, Level::Low);//  Level::Low);
    //let mut rst = Output::new(io.pins.gpio8, Level::Low);

    let sck = Output::new(peripherals.GPIO40,  Level::Low);//  Level::Low);
    let miso = Output::new(peripherals.GPIO38,  Level::Low);
    let mosi = Output::new(peripherals.GPIO41,  Level::Low);
    let cs = Output::new(peripherals.GPIO12,  Level::Low);
    //let cs = io.pins.gpio10.into_push_pull_output();

    let spi = Spi::new(peripherals.SPI2,Config::default(),).unwrap().with_miso(miso).with_mosi(mosi).with_sck(sck);
    let config = Config::default().with_frequency(62500.kHz()).with_mode(Mode::_0); 

    let spi_device = embedded_hal_bus::spi::ExclusiveDevice::new(spi, cs, delay).unwrap();

    let mut buffer = [0_u8; 512];
    let di = SpiInterface::new(spi_device, dc, &mut buffer);

    let mut display = Builder::new(ST7789, di).display_size(240, 320).invert_colors(ColorInversion::Inverted).init(&mut delay).unwrap();

    display.clear(Rgb565::GREEN).unwrap();

    let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::GREEN)
        .build();

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
    loop {

        let style = embedded_graphics::primitives::PrimitiveStyleBuilder::new()
        .fill_color(Rgb565::WHITE)
        .build();

        embedded_graphics::primitives::Rectangle::new(Point::zero(), display.bounding_box().size)
        .into_styled(style)
        .draw(&mut display)
        .unwrap();

        info!("Hello world!");
        delay.delay_millis(500);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.23.1/examples/src/bin
}
