use embassy_executor;
use embassy_nrf::{peripherals::USBD, usb};

pub type Vbus = usb::vbus_detect::SoftwareVbusDetect;
pub type UsbDrv<'d> = usb::Driver<'d, USBD, &'d Vbus>;


const LOG_BUF: usize = 1024;
const LOG_LEVEL: log::LevelFilter = log::LevelFilter::Info;



#[embassy_executor::task]
pub async fn run(driver: UsbDrv<'static>) -> ! {
    embassy_usb_logger::run!(LOG_BUF, LOG_LEVEL, driver);
}

