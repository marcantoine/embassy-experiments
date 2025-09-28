#![no_std]
#![no_main]

mod fmt;
mod usb_logger;

use panic_halt as _;
use fmt::info;

use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals::USBD, usb};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::Timer;
use static_cell::StaticCell;
use usb_logger::{Vbus, UsbDrv};


static VBUS_DETECT: StaticCell<Vbus> = StaticCell::new();

bind_interrupts!(struct Irqs {
    // Interrupt handler for the USBD peripheral.
    USBD => usb::InterruptHandler<USBD>;
    // Interrupt handler for the temperature sensor.
    TEMP => embassy_nrf::temp::InterruptHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());
    let mut led = Output::new(p.P0_06, Level::Low, OutputDrive::Standard);


    let vbus_mut = VBUS_DETECT.init(Vbus::new(true, true));
    let driver: UsbDrv<'static> = usb::Driver::new(p.USBD, Irqs, &*vbus_mut);

    // Start USB logging in a background task.
    spawner.spawn(usb_logger::run(driver)).unwrap();

    let mut temp = embassy_nrf::temp::Temp::new(p.TEMP, Irqs);


    loop {
        let t = temp.read().await;
        info!("Die temperature: {} °C", t);
        led.set_high();
        Timer::after_millis(9500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
