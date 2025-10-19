#![no_std]
#![no_main]


use defmt::info;
use defmt_rtt as _; 
use embassy_executor::Spawner;
use embassy_nrf::{gpio::{ Output, Level, OutputDrive }};
use embassy_time::Timer;
use panic_probe as _;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let _col = Output::new(p.P0_28, Level::Low, OutputDrive::Standard);
    let mut row = Output::new(p.P0_21, Level::Low, OutputDrive::Standard);

    loop{
        info!("blink");
        row.set_high();
        Timer::after_millis(100).await;
        row.set_low();
        Timer::after_millis(100).await;
    }

}