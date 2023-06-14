use std::time::Duration;
use std::thread;

use rusb::{
    Context, Device, DeviceDescriptor, DeviceHandle, Direction, Result, TransferType, UsbContext,
};

#[derive(Debug)]
struct Endpoint {
    config: u8,
    iface: u8,
    setting: u8,
    address: u8,
}

fn convert_argument(input: &str) -> u16 {
    if input.starts_with("0x") {
        return u16::from_str_radix(input.trim_start_matches("0x"), 16).unwrap();
    }
    u16::from_str_radix(input, 10)
        .expect("Invalid input, be sure to add `0x` for hexadecimal values.")
}

fn open_device<T: UsbContext>(
    context: &mut T,
    vid: u16,
    pid: u16,
) -> Option<(Device<T>, DeviceDescriptor, DeviceHandle<T>)> {
    let devices = match context.devices() {
        Ok(d) => d,
        Err(_) => return None,
    };

    for device in devices.iter() {
        let device_desc = match device.device_descriptor() {
            Ok(d) => d,
            Err(_) => continue,
        };

        if device_desc.vendor_id() == vid && device_desc.product_id() == pid {
            match device.open() {
                Ok(handle) => return Some((device, device_desc, handle)),
                Err(e) => panic!("Device found but failed to open: {}", e),
            }
        }
    }

    None
}

fn find_readable_endpoint<T: UsbContext>(
    device: &mut Device<T>,
    device_desc: &DeviceDescriptor,
    transfer_type: TransferType,
) -> Option<Endpoint> {
    for n in 0..device_desc.num_configurations() {
        let config_desc = match device.config_descriptor(n) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for interface in config_desc.interfaces() {
            for interface_desc in interface.descriptors() {
                for endpoint_desc in interface_desc.endpoint_descriptors() {
                    if endpoint_desc.direction() == Direction::In
                        && endpoint_desc.transfer_type() == transfer_type
                    {
                        return Some(Endpoint {
                            config: config_desc.number(),
                            iface: interface_desc.interface_number(),
                            setting: interface_desc.setting_number(),
                            address: endpoint_desc.address(),
                        });
                    }
                }
            }
        }
    }

    None
}

fn configure_endpoint<T: UsbContext>(
    handle: &mut DeviceHandle<T>,
    endpoint: &Endpoint,
) -> Result<()> {
    handle.set_active_configuration(endpoint.config)?;
    handle.claim_interface(endpoint.iface)?;
    handle.set_alternate_setting(endpoint.iface, endpoint.setting)?;
    Ok(())
}

fn main() {

    // args
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("usage: read_device <base-10/0xbase-16> <base-10/0xbase-16>");
        return;
    }

    let vid = convert_argument(args[1].as_ref());
    let pid = convert_argument(args[2].as_ref());
    // println!("vid: {:?}, pid: {:?}",&args[1],&args[1]);
    // println!("vid: {:?}, pid: {:?}",&vid,&pid);

    // create context
    let mut context = match rusb::Context::new() {
        Ok(c) => c,
        Err(e) => panic!("libusb::Context::new(): {}", e),
    };
    //println!("context: {:?}",&context);
    
    //context.set_log_level(rusb::LogLevel::Debug);
    //context.set_log_level(rusb::LogLevel::Info);
    //context.set_log_level(rusb::LogLevel::Warning);
    //context.set_log_level(rusb::LogLevel::Error);
    //context.set_log_level(rusb::LogLevel::None);


    // open device
    let (mut device, device_desc, mut handle) = open_device(&mut context, vid, pid).unwrap();
    //println!("device: {:?}\ndevice_desc: {:?}\nhandle: {:?}\n",&device, &device_desc, &handle);

    // get endpoint
    let endpoint: Endpoint = find_readable_endpoint(&mut device, &device_desc, TransferType::Bulk).unwrap();
    println!("endpoint: {:?}",&endpoint);

    // make sure to release driver to make it available to read
    let has_kernel_driver = match handle.kernel_driver_active(endpoint.iface) {
        Ok(true) => {
            handle.detach_kernel_driver(endpoint.iface).ok();
            true
        }
        _ => false,
    };

    // configure endpoint
    println!("Reading from endpoint: {:?}", &endpoint);
    match configure_endpoint(&mut handle, &endpoint) {

        Ok(_) => {
            let mut buf = [0; 256];
            let timeout = Duration::from_secs(2);



            // continously receive data
            loop {
                thread::sleep(Duration::from_secs(1));
                buf = [0; 256];

                match handle.read_bulk(endpoint.address, &mut buf, timeout) {
                    Ok(len) => {
                        //println!(" - read: {:?}", &buf[..len]);

                            let s = match std::str::from_utf8(&buf) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),

                        };

                        println!("{}", s);
                    }
                    Err(err) => println!("could not read from endpoint: {}", err),
                }
                
            }




        }
        Err(err) => println!("could not configure endpoint: {}", err),
    }
}