
use usb_device::bus::{UsbBus, UsbBusAllocator};
use usb_device::device::{UsbDevice, UsbDeviceBuilder, UsbVidPid};
use usb_device::prelude::BuilderError;
use usb_device::UsbError;
use usbd_serial::SerialPort;
pub struct CDCDevice<'a, B: UsbBus> {
    usb_device: UsbDevice<'a, B>,
    serial_port: SerialPort<'a, B>,
}

impl<'a, B: UsbBus> CDCDevice<'a, B> {
    pub fn init(alloc_ref: &'a mut UsbBusAllocator<B>, vid: u16, pid: u16, manufacturer: &'a str, product: &'a str, sn: &'a str) -> Result<CDCDevice<'a, B>, BuilderError> {
        let serial = SerialPort::new(alloc_ref);
        let usb_desc = usb_device::device::StringDescriptors::default()
            .manufacturer(manufacturer)
            .product(product)
            .serial_number(sn);

        let usb_dev = UsbDeviceBuilder::new(alloc_ref, UsbVidPid(vid, pid))
            .device_class(usbd_serial::USB_CLASS_CDC)
            .strings(&[usb_desc])?
            .build();

        Ok(Self{
            usb_device: usb_dev,
            serial_port: serial
        })
    }

    pub fn poll(&mut self) {
        if !self.usb_device.poll(&mut [&mut self.serial_port]) {
            return;
        }
    }

    pub fn read(&mut self, buf: &mut [u8]) -> usize {
        match self.serial_port.read(buf) {
            Ok(count) if count > 0 => count,
            _ => {0}
        }
    }

    pub fn write_byte(&mut self, data: u8) -> Result<usize, UsbError> {
        self.serial_port.write(&[data])
    }

    pub fn flush(&mut self) {
        let _ = self.serial_port.flush();
    }
}