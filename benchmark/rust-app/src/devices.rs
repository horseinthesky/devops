use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Device<'a> {
    uuid: Uuid,
    mac: &'a str,
    firmware: &'a str,
}

pub fn get_devices() -> Vec<Device<'static>> {
    vec![
        Device {
            uuid: Uuid::new_v4(),
            mac: "5F-33-CC-1F-43-82",
            firmware: "2.1.6",
        },
        Device {
            uuid: Uuid::new_v4(),
            mac: "EF-2B-C4-F5-D6-34",
            firmware: "2.1.5",
        },
        Device {
            uuid: Uuid::new_v4(),
            mac: "62-46-13-B7-B3-A1",
            firmware: "3.0.0",
        },
        Device {
            uuid: Uuid::new_v4(),
            mac: "96-A8-DE-5B-77-14",
            firmware: "1.0.1",
        },
        Device {
            uuid: Uuid::new_v4(),
            mac: "7E-3B-62-A6-09-12",
            firmware: "3.5.6",
        },
    ]
}
