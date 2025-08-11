use super::scinterface::SCNetworkInterface;
use crate::sys::{dummy_socket, ioctls, InterfaceHandle};
use crate::sys::{ifreq, InterfaceName};
use crate::{Error, Interface};
use ipnet::IpNet;
use nix::sys::socket::{SockaddrIn, SockaddrIn6};
use std::net;
use std::os::unix::io::AsRawFd;

pub trait InterfaceExt {
    fn set_up(&self, v: bool) -> Result<(), Error>;
    fn set_running(&self, v: bool) -> Result<(), Error>;
    fn alias(&self) -> Result<String, Error>;
}

impl InterfaceHandle {
    pub fn add_address(&self, network: IpNet) -> Result<(), Error> {
        let socket = dummy_socket()?;
        let name = self.name()?;
        match network {
            IpNet::V4(network) => {
                let ifra_addr = SockaddrIn::from(net::SocketAddrV4::new(network.addr(), 0));
                let ifra_dest_addr = SockaddrIn::from(net::SocketAddrV4::new(network.network(), 0));
                let ifra_dest_mask = SockaddrIn::from(net::SocketAddrV4::new(network.netmask(), 0));

                let req = ifreq::ifaliasreq4 {
                    ifra_name: name
                        .parse::<InterfaceName>()
                        .map_err(|e| std::io::Error::other(e.to_string()))?,
                    ifra_addr: *ifra_addr.as_ref(),
                    ifra_broadaddr: *ifra_dest_addr.as_ref(),
                    ifra_mask: *ifra_dest_mask.as_ref(),
                };

                unsafe {
                    ioctls::siocaifaddr4(socket.as_raw_fd(), &req)?;
                }
                Ok(())
            }
            IpNet::V6(network) => {
                let ifra_addr = SockaddrIn6::from(net::SocketAddrV6::new(network.addr(), 0, 0, 0));
                let ifra_dest_addr =
                    SockaddrIn6::from(net::SocketAddrV6::new(network.network(), 0, 0, 0));
                let ifra_dest_mask =
                    SockaddrIn6::from(net::SocketAddrV6::new(network.netmask(), 0, 0, 0));

                let req = ifreq::ifaliasreq6 {
                    ifra_name: name
                        .parse::<InterfaceName>()
                        .map_err(|e| std::io::Error::other(e.to_string()))?,
                    ifra_addr: *ifra_addr.as_ref(),
                    ifra_broadaddr: *ifra_dest_addr.as_ref(),
                    ifra_mask: *ifra_dest_mask.as_ref(),
                };

                unsafe {
                    ioctls::siocaifaddr6(socket.as_raw_fd(), &req)?;
                }
                Ok(())
            }
        }
    }

    pub fn remove_address(&self, _network: IpNet) -> Result<(), Error> {
        todo!()
    }

    pub fn hwaddress(&self) -> Result<[u8; 6], Error> {
        todo!()
    }
}

impl InterfaceExt for Interface {
    fn alias(&self) -> Result<String, Error> {
        match SCNetworkInterface::get_displayname(&self.name()?) {
            Some(alias) => Ok(alias),
            None => Ok(self.name()?),
        }
    }
    fn set_up(&self, v: bool) -> Result<(), Error> {
        self.0.set_up(v)
    }
    fn set_running(&self, v: bool) -> Result<(), Error> {
        self.0.set_running(v)
    }
}
