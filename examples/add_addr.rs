use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use netconfig_rs::{Interface, ipnet};
fn main()->Result<(),Box<dyn Error>>{
    let interface_index = 0;
    let interface = unsafe {Interface::from_index_unchecked(interface_index)};
    let addrs = interface.addresses()?;
    println!("before {:#?}", addrs);
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10,6,0,1]).into(),24)?)?;
    interface.add_address(ipnet::IpNet::new(Ipv4Addr::from([10,5,0,1]).into(),24)?)?;
    let addrs = interface.addresses()?;
    println!("after {:#?}", addrs);
    Ok(())
}