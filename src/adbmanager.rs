use crate::device::Device;
use crate::device::DeviceStatus;

use std::{cell::RefCell, collections::HashMap, io::Error, process::Command, str::FromStr};

pub struct AdbManager {
    devices: RefCell<HashMap<String, Device>>,
}

impl AdbManager {
    pub fn new() -> AdbManager {
        AdbManager {
            devices: RefCell::new(HashMap::new()),
        }
    }

    pub fn devices(&self) -> Vec<Device> {
        self.devices.borrow().values().cloned().collect()
    }

    pub fn resync(&mut self) {
        let mut borrowed_devices = self.devices.borrow_mut();
        borrowed_devices.clear();
        let mut command = Command::new("adb");
        command.args(["devices", "-l"]);
        let output = String::from_utf8(command.output().unwrap().stdout).unwrap();
        let lines = output
            .split_once("List of devices attached\n")
            .unwrap()
            .1
            .lines();
        //lines.next(); //skip 1st line

        //example:
        //192.168.1.31:5555      device product:uzw4010tim model:TIM_BOX device:uzw4010tim transport_id:1

        for line in lines {
            if line.len() == 0 {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect(); //split each line by whitespace
            let serial = String::from(parts[0]); //get serial (ip_address:port)
            if borrowed_devices.contains_key(&serial) {
                continue;
            }
            let status = DeviceStatus::from_str(parts[1]).unwrap(); //get device status

            //parse transport id from last element
            let transport_id = Some(
                parts
                    .last()
                    .unwrap()
                    .split_once(':')
                    .unwrap()
                    .1
                    .parse::<u16>()
                    .unwrap(),
            );
            // something better? idk good enough for now
            borrowed_devices.insert(
                serial.clone(),
                Device {
                    serial,
                    status,
                    transport_id,
                },
            );
        }
    }

    pub fn pair(address: &str, pairing_code: &str) -> Result<(), Error> {
        match Command::new("adb")
            .args(["pair", address, pairing_code])
            .output()
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn connect(&mut self, address: &str) -> Result<(), Error> {
        match Command::new("adb").args(["connect", address]).output() {
            Ok(_) => {
                self.devices
                    .borrow_mut()
                    .insert(address.to_owned(), Device::new(address).unwrap());
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn disconnect(&mut self, device: &Device) -> Result<(), Error> {
        match Command::new("adb")
            .args(["disconnect", &device.serial])
            .output()
        {
            Ok(_) => {
                self.devices.borrow_mut().remove(&device.serial);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn disconnect_all(&mut self) -> Result<(), Error> {
        match Command::new("adb").args(["disconnect"]).output() {
            Ok(_) => {
                self.devices.borrow_mut().clear();
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
