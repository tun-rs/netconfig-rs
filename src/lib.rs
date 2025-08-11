mod error;
use advmac::MacAddr6;
pub use error::Error;
pub use ipnet;
use ipnet::IpNet;
use std::collections::HashSet;

pub mod sys;

/// Wrapped interface index.
///
/// Index is chosen, because basically all operating systems use index as an identifier.
/// This struct can be used to manipulate interface parameters, such as IP address and MTU.
#[derive(Debug)]
pub struct Interface(sys::InterfaceHandle);

impl Interface {
    /// Add address to the interface
    pub fn add_address(&self, network: IpNet) -> Result<(), Error> {
        self.0.add_address(network)
    }
    /// Remove the specified address from the interface
    pub fn remove_address(&self, network: IpNet) -> Result<(), Error> {
        self.0.remove_address(network)
    }
    /// Returns list of IP addresses, assigned to this Interface
    pub fn addresses(&self) -> Result<Vec<IpNet>, Error> {
        self.0.addresses()
    }
    pub fn mtu(&self) -> Result<u32, Error> {
        self.0.mtu()
    }
    pub fn set_mtu(&self, mtu: u32) -> Result<(), Error> {
        self.0.set_mtu(mtu)
    }
    #[cfg(windows)]
    pub fn set_mtu_v4(&self, mtu: u32) -> Result<(), Error> {
        self.0.set_mtu_v4(mtu)
    }
    #[cfg(windows)]
    pub fn set_mtu_v6(&self, mtu: u32) -> Result<(), Error> {
        self.0.set_mtu_v6(mtu)
    }
    pub fn name(&self) -> Result<String, Error> {
        self.0.name()
    }
    pub fn index(&self) -> Result<u32, Error> {
        self.0.index()
    }
    /// Returns MAC address, assigned to this Interface
    pub fn hwaddress(&self) -> Result<MacAddr6, Error> {
        self.0.hwaddress()
    }
    /// # Safety
    /// The passed interface index must be valid
    pub unsafe fn from_index_unchecked(index: u32) -> Self {
        Self(sys::InterfaceHandle { index })
    }
    /// Returns `InterfaceHandle` from given interface index or Error if not found.
    ///
    /// This method checks given index for validity and interface for presence. If you want to get
    /// `InterfaceHandle` without checking interface for presence, use [`from_index_unchecked`](Self::from_index_unchecked).
    pub fn try_from_index(index: u32) -> Result<Self, Error> {
        sys::InterfaceHandle::try_from_index(index)
    }
    /// Returns `InterfaceHandle` from given name or Error if not found.
    ///
    /// On Windows it uses interface name, that is similar to `ethernet_32774`.
    /// If you want to search interface by human-readable name (like `Ethernet 1`), use `try_from_alias`
    pub fn try_from_name(name: &str) -> Result<Self, Error> {
        sys::InterfaceHandle::try_from_name(name)
    }
}

pub fn list_interfaces() -> Result<Vec<Interface>, Error> {
    sys::list_interfaces()
}

pub fn list_addresses() -> Result<Vec<IpNet>, Error> {
    let interfaces = list_interfaces()?;

    let addresses = interfaces
        .iter()
        .flat_map(|iface| iface.addresses())
        .flatten();

    Ok(HashSet::<IpNet>::from_iter(addresses)
        .iter()
        .cloned()
        .collect())
}
