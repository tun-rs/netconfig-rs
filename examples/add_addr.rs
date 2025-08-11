use netconfig_rs::{ipnet, Interface};
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
fn main() -> Result<(), Box<dyn Error>> {
    let interface_name = "tun0";
    let interface = Interface::try_from_name(interface_name)?;
    let addrs = interface.addresses()?;
    println!("before {:#?}", addrs);
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10, 6, 0, 1]).into(), 24)?)?;
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10, 5, 0, 1]).into(), 24)?)?;
    interface.add_address(ipnet::IpNet::new(
        Ipv6Addr::from([
            0xCDCD, 0x910A, 0x2222, 0x5498, 0x8475, 0x1111, 0x3900, 0x2021,
        ])
        .into(),
        24,
    )?)?;
    let addrs = interface.addresses()?;
    println!("after {:#?}", addrs);
    println!("remove addr:");
    interface.remove_address(ipnet::IpNet::new(Ipv4Addr::from([10, 5, 0, 1]).into(), 24)?)?;
    let addrs = interface.addresses()?;
    println!("final {:#?}", addrs);
    Ok(())
}
