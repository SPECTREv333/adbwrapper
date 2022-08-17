use std::{
    io::{Error, ErrorKind},
    process::{Command, Output},
    str::FromStr,
};

#[derive(Clone)]
pub enum DeviceStatus {
    Offline,
    Device,
    Unauthorized,
}

impl FromStr for DeviceStatus {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "device" => Ok(DeviceStatus::Device),
            "offline" => Ok(DeviceStatus::Offline),
            "unauthorized" => Ok(DeviceStatus::Unauthorized),
            _ => Err(Error::new(
                ErrorKind::InvalidInput,
                "Unexpected input to convert to DeviceStatus",
            )),
        }
    }
}

#[derive(Clone)]
pub struct Device {
    pub serial: String,
    pub status: DeviceStatus,
    pub transport_id: Option<u16>,
}

impl Device {
    pub fn new(address: &str) -> Result<Device, Error> {
        match Command::new("adb")
            .args(["-s", address, "get-state"])
            .output()
        {
            Ok(output) => {
                let status =
                    DeviceStatus::from_str(&String::from_utf8(output.stdout).unwrap()).unwrap();
                Ok(Device {
                    serial: address.to_owned(),
                    status,
                    transport_id: None,
                })
            }
            Err(e) => Err(e),
        }
    }

    pub fn push(&self, local: &str, remote: &str) -> Result<Output, Error> {
        match Command::new("adb")
            .args(["-s", &self.serial, "push", local, remote])
            .output()
        {
            Ok(output) => Ok(output),
            Err(e) => Err(e),
        }
    }

    pub fn pull(&self, remote: &str, local: &str) -> Result<Output, Error> {
        match Command::new("adb")
            .args(["-s", &self.serial, "pull", remote, local])
            .output()
        {
            Ok(output) => Ok(output),
            Err(e) => Err(e),
        }
    }

    pub fn shell_command(&self, command: &str) -> Result<Output, Error> {
        match Command::new("adb")
            .args(["-s", &self.serial, "shell", command])
            .output()
        {
            Ok(output) => Ok(output),
            Err(e) => Err(e),
        }
    }
}
