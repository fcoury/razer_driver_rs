extern crate hidapi;
pub mod razer_device;
pub mod razer_keyboard;
pub mod razer_report;

use associated::Associated;
use hidapi::HidApi;
use razer_device::RazerDevice;
use razer_keyboard::RazerKeyboardKind;
use std::error::Error;

#[derive(Default)]
pub struct FoundRazerDevices {
    pub keyboards: Vec<RazerDevice<RazerKeyboardKind>>,
}

pub fn scan_for_devices() -> Result<FoundRazerDevices, Box<dyn Error>> {
    let mut devices = FoundRazerDevices::default();
    let api = HidApi::new()?;
    for device in api.device_list() {
        if device.vendor_id() == razer_device::RAZER_VENDOR_ID {
            if let Some(valid_device) = RazerKeyboardKind::from_repr(device.product_id()) {
                let connect_info = valid_device.get_associated();
                if (connect_info.interface_number == None
                    || connect_info.interface_number == Some(device.interface_number()))
                    && (connect_info.usage == None || connect_info.usage == Some(device.usage()))
                    && (connect_info.usage_page == None
                        || connect_info.usage_page == Some(device.usage_page()))
                {
                    println!(
                        "found {:?}, {:#04x}, {}, {}, {}, {}",
                        device.serial_number(),
                        device.product_id(),
                        device.interface_number(),
                        device.usage(),
                        device.usage_page(),
                        device.path().to_string_lossy(),
                    );
                    if let Ok(hid_device) = device.open_device(&api) {
                        devices
                            .keyboards
                            .push(RazerDevice::new(valid_device, hid_device));
                    }
                }
            }
        }
    }
    Ok(devices)
}

#[cfg(test)]
mod tests {
    use crate::scan_for_devices;

    #[test]
    fn it_works() {
        let devices = scan_for_devices().unwrap();
        assert_eq!(devices.keyboards.len(), 1);
        let keyboard = devices.keyboards.get(0).unwrap();
        let version = keyboard.get_firmware_version().unwrap();
        println!("{}", version);
        assert_eq!("v2.1", version.to_string());
        println!("brightness {}", keyboard.get_brightness().unwrap());

        keyboard.set_brightness(90).unwrap();
        let brightness = keyboard.get_brightness().unwrap();
        println!("brightness {}", brightness);
        assert_eq!(90, brightness);
    }
}
