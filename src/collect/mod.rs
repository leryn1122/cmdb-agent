use std::io::Result;

use smbioslib::CoreCount;
use smbioslib::CoreCount2;
use smbioslib::CoresEnabled;
use smbioslib::CoresEnabled2;
use smbioslib::MemorySize;
use smbioslib::MemorySizeExtended;
use smbioslib::ProcessorSpeed;
use smbioslib::SMBiosData;
use smbioslib::SMBiosMemoryDevice;
use smbioslib::SMBiosProcessorInformation;
use smbioslib::ThreadCount;
use smbioslib::ThreadCount2;

use crate::collect::host::get_hostname;
use crate::collect::smbios::opt::BiosType;
use crate::collect::smbios::opt::Keyword;
use crate::schema;

pub mod host;
pub mod http;
pub mod net;
pub mod smbios;
pub mod task;

pub fn get_machine_info() -> Result<schema::MachineInfo> {
  let mut machine_info = schema::MachineInfo::default();
  let smbios = smbios::get_smbios_data()?;

  // Hostname
  machine_info.set_hostname(get_hostname()?);

  // Serial number
  machine_info.set_serial_number(get_serial_number(&smbios)?);

  // OS
  machine_info.set_os(get_os()?);

  // Devices
  machine_info.set_devices(get_devices(&smbios)?);

  // Network
  machine_info.set_networks(get_networks()?);

  Ok(machine_info)
}

fn get_serial_number(smbios: &SMBiosData) -> Result<String> {
  Keyword::SystemSerialNumber
    .parse(smbios)
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))
}

fn get_os() -> Result<schema::OS> {
  let mut os = schema::OS::default();
  let uname = host::uname()?;
  let map = host::get_os_release()?;

  os.set_platform(std::env::consts::OS.to_string());
  os.set_os(map.get("NAME").unwrap().to_string());
  os.set_version(map.get("VERSION").unwrap().to_string());
  os.set_arch(std::env::consts::ARCH.to_string());
  os.set_kernel(uname.release);

  Ok(os)
}

fn get_devices(smbios: &SMBiosData) -> Result<schema::Devices> {
  let mut devices = schema::Devices::default();

  devices.set_processor(get_processor(smbios)?);
  devices.set_memory(get_memory(smbios)?);

  Ok(devices)
}

fn get_processor(smbios: &SMBiosData) -> Result<Vec<schema::Processor>> {
  let mut results = vec![];

  let processors: Vec<SMBiosProcessorInformation> =
    BiosType::parse_vec(&BiosType::Processor, smbios);
  for processor in processors {
    let mut result = schema::Processor::default();

    result.set_version(processor.processor_version().to_string());
    result.set_max_speed(match processor.max_speed().unwrap() {
      ProcessorSpeed::Unknown => 0,
      ProcessorSpeed::MHz(s) => s,
    });
    result.set_current_speed(match processor.current_speed().unwrap() {
      ProcessorSpeed::Unknown => 0,
      ProcessorSpeed::MHz(s) => s,
    });
    result.set_core_count(match processor.core_count().unwrap() {
      CoreCount::Unknown => 0,
      CoreCount::Count(c) => c.into(),
      CoreCount::SeeCoreCount2 => match processor.core_count_2().unwrap() {
        CoreCount2::Unknown => 0,
        CoreCount2::Count(c) => c,
        CoreCount2::Reserved => 0,
      },
    });
    result.set_cores_enabled(match processor.cores_enabled().unwrap() {
      CoresEnabled::Unknown => 0,
      CoresEnabled::Count(c) => c.into(),
      CoresEnabled::SeeCoresEnabled2 => match processor.cores_enabled_2().unwrap() {
        CoresEnabled2::Unknown => 0,
        CoresEnabled2::Count(c) => c,
        CoresEnabled2::Reserved => 0,
      },
    });
    result.set_thread_count(match processor.thread_count().unwrap() {
      ThreadCount::Unknown => 0,
      ThreadCount::Count(c) => c.into(),
      ThreadCount::SeeThreadCount2 => match processor.thread_count_2().unwrap() {
        ThreadCount2::Unknown => 0,
        ThreadCount2::Count(c) => c,
        ThreadCount2::Reserved => 0,
      },
    });

    results.push(result);
  }
  Ok(results)
}

fn get_memory(smbios: &SMBiosData) -> Result<schema::Memory> {
  let mut result = schema::Memory::default();

  let memories: Vec<SMBiosMemoryDevice> = BiosType::parse_vec(&BiosType::Numeric(17), smbios);
  let total_memory: u64 = memories
    .iter()
    .map(|memory| match memory.size().unwrap() {
      MemorySize::NotInstalled => 0,
      MemorySize::Unknown => 0,
      MemorySize::SeeExtendedSize => match memory.extended_size().unwrap() {
        MemorySizeExtended::Megabytes(mb) => (mb as u64) * 1024 * 1024,
        MemorySizeExtended::SeeSize => unreachable!("Never see `SeeSize` if `SeeExtendedSize`"),
      },
      MemorySize::Kilobytes(kb) => (kb as u64) * 1024,
      MemorySize::Megabytes(mb) => (mb as u64) * 1024 * 1024,
    })
    .sum();

  result.set_total_memory(total_memory);
  result.set_unit("B".to_string());

  Ok(result)
}

fn get_networks() -> Result<Vec<schema::Network>> {
  let networks = net::get_net_ifaces()
    .iter()
    .map(|iface| {
      let mut network = schema::Network::default();
      network.set_name(iface.name.clone());
      for ipv4 in &iface.ipv4 {
        network.address_mut().push(ipv4.to_string());
      }
      for ipv6 in &iface.ipv6 {
        network.address_mut().push(ipv6.to_string());
      }
      network.set_mac_address(iface.mac_addr.unwrap().to_string());
      network
    })
    .collect();

  Ok(networks)
}

#[cfg(test)]
mod tests {
  use crate::collect::get_machine_info;

  #[test]
  fn test_get_machine_info() {
    let info = get_machine_info().expect("Machine info");
    println!("{:?}", info);
  }
}
