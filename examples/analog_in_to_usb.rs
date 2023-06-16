let mut serial = SerialPort::new(&usb_bus);

let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
    .product("Serial port")
    .device_class(USB_CLASS_CDC)
    .build();

loop {
    if !usb_dev.poll(&mut [&mut serial]) {
        continue;
    }

    let mut buf = [0u8; 64];

    match serial.read(&mut buf[..]) {
        Ok(count) => {
            // count bytes were read to &buf[..count]
        },
        Err(UsbError::WouldBlock) => // No data received
        Err(err) => // An error occurred
    };

    match serial.write(&[0x3a, 0x29]) {
        Ok(count) => {
            // count bytes were written
        },
        Err(UsbError::WouldBlock) => // No data could be written (buffers full)
        Err(err) => // An error occurred
    };
}