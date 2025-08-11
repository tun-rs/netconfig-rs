use netconfig_rs::{ipnet, Interface};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
fn main() -> Result<(), Box<dyn Error>> {
    let interface_name = "tun0";
    let interface = Interface::try_from_name(interface_name)?;
    let addrs = interface.addresses()?;
    println!("before {:#?}", addrs);
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10, 6, 0, 1]).into(), 24)?)?;
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10, 5, 0, 1]).into(), 24)?)?;
    let addrs = interface.addresses()?;
    println!("after {:#?}", addrs);
    Ok(())
}
