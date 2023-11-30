enum SerialType {
    USB,
    Bluetooth,
    RS232,
}
enum SerialState {
    Open,
    Close,
}

enum BaudRate {
    B9600,
    B19200,
    B38400,
    B57600,
    B115200,
    B230400,
    B460800,
    B921600,
}

#[derive(Debug, Clone)]
struct Device {
    name: String,
    display_name: String,
    path: String,
    baud: BaudRate,
    dev_type: SerialType,
    state: SerialState,
}
#[derive(Debug, Clone)]
struct Serial {
    devices: Vec<Device>,
}

impl Serial {
    fn new() -> Self {
        Self {
            name: String::new(),
            display_name: String::new(),
            path: String::new(),
            baud: BaudRate::B9600,
            dev_type: SerialType::USB,
            state: SerialState::Close,
        }
    }
    fn get_devices(&mut self) -> Vec<Device> {
        let mut devices = Vec::new();
        let mut device = Device::new();
        device.name = "USB".to_string();
        device.display_name = "USB".to_string();
        device.path = "/dev/ttyUSB0".to_string();
        device.baud = BaudRate::B9600;
        device.dev_type = SerialType::USB;
        device.state = SerialState::Close;
        devices.push(device);
        devices
    }
}
