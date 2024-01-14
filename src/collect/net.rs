use default_net::Interface;

#[cfg(target_os = "linux")]
pub const VETH_SYS_FS_PATH: &str = "/sys/devices/virtual/net";

pub fn get_net_ifaces() -> Vec<Interface> {
  default_net::get_default_interface()
    .into_iter()
    .filter(|iface| iface.is_loopback())
    .filter(|iface| iface.is_up())
    .filter(veth_interface_filter)
    .filter(has_ip_iface_filter)
    .collect()
}

#[cfg(target_os = "linux")]
pub fn veth_interface_filter(iface: &Interface) -> bool {
  let veths: Vec<_> = std::fs::read_dir(VETH_SYS_FS_PATH)
    .unwrap_or_else(|_| panic!("Failed to visit sysfs on {}", VETH_SYS_FS_PATH))
    .map(|f| f.expect("").file_name())
    .map(|s| s.to_str().unwrap().to_string())
    .collect();
  !veths.contains(&iface.name)
}

#[cfg(not(target_os = "linux"))]
pub fn veth_interface_filter(iface: &Interface) -> bool {
  true
}

pub fn has_ip_iface_filter(iface: &Interface) -> bool {
  !iface.ipv4.is_empty() || !iface.ipv6.is_empty()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_interfaces() {
    let iface = get_net_ifaces();
    println!("{:?}", iface);
  }
}
