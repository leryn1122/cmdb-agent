use std::collections::HashMap;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;
use std::str::FromStr;

use smbioslib::SMBiosData;
use smbioslib::SMBiosEntryPoint64;
use smbioslib::SMBiosVersion;
use smbioslib::UndefinedStructTable;
use strum::IntoEnumIterator;

use crate::support::smbios::opt::Keyword;

pub mod error;
pub mod opt;

pub(crate) fn collect_dmidecode() -> std::io::Result<String> {
  let smbios_data = table_load()?;
  let mut map = HashMap::<String, String>::new();
  let keywords: Vec<&str> = Keyword::iter().map(|v| v.into()).collect();
  for keyword in keywords {
    map.insert(
      keyword.to_string(),
      Keyword::from_str(keyword)
        .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?
        .parse(&smbios_data)
        .unwrap(),
    );
  }
  Ok(format!("{:?}\n", map))
}

#[cfg(target_os = "macos")]
fn table_load() -> std::io::Result<SMBiosData> {
  log::trace!("Scanning SMBios by IOKit.");
  smbioslib::table_load_from_device()
}

#[cfg(target_os = "linux")]
fn table_load() -> std::io::Result<SMBiosData> {
  table_load_from_sys()
}

#[cfg(target_os = "linux")]
fn table_load_from_sys() -> std::io::Result<SMBiosData> {
  log::trace!(
    "Scanning [{}] for SMBios entry point",
    smbioslib::SYS_ENTRY_FILE
  );

  let version: SMBiosVersion;
  let sys_entry_path = Path::new(smbioslib::SYS_ENTRY_FILE);
  SMBiosEntryPoint64::try_load_from_file(sys_entry_path).map(|entry_point| {
    version = SMBiosVersion {
      major:    entry_point.major_version(),
      minor:    entry_point.minor_version(),
      revision: entry_point.docrev(),
    };

    log::trace!(
      "Visit SMBios (version {}.{}.{}) Table at {:#010X}.",
      &version.major,
      &version.minor,
      &version.revision,
      &entry_point.structure_table_address()
    );
  })?;

  log::trace!("Load SMBios structures from {}", smbioslib::SYS_TABLE_FILE);
  let smbios_data = SMBiosData::try_load_from_file(smbioslib::SYS_TABLE_FILE, Some(version))?;
  Ok(smbios_data)
}

#[cfg(target_os = "linux")]
fn table_load_from_dev_mem() -> std::io::Result<SMBiosData> {
  log::trace!(
    "Scanning [{}] for SMBios entry point",
    smbioslib::SYS_ENTRY_FILE
  );

  const RANGE_START: u64 = 0x000F0000u64;
  const RANGE_END: u64 = 0x000FFFFFFu64;

  let mut dev_mem = std::fs::File::open(smbioslib::DEV_MEM_FILE)?;
  let structure_table_addr: u64;
  let structure_table_length: u32;
  let version: SMBiosVersion;

  match SMBiosEntryPoint64::try_scan_from_file(&mut dev_mem, RANGE_START..=RANGE_END) {
    Ok(entry_point) => {
      structure_table_addr = entry_point.structure_table_address() as u64;
      structure_table_length = entry_point.structure_table_maximum_size() as u32;
      version = SMBiosVersion {
        major:    entry_point.major_version(),
        minor:    entry_point.minor_version(),
        revision: entry_point.docrev(),
      };

      log::trace!(
        "Visit SMBios (version {}.{}.{}) Table at {:#010X}.",
        &version.major,
        &version.minor,
        &version.revision,
        &entry_point.structure_table_address()
      );
      log::trace!(
        "SMBios structures occupying {} bytes",
        entry_point.structure_table_maximum_size()
      );
    }
    Err(err) => {
      return Err(err)?;
    }
  };

  if structure_table_addr + structure_table_length as u64 > RANGE_END {
    return Err(Error::new(
      ErrorKind::InvalidData,
      format!(
        "The entry point has given a length which exceeds the range: {}",
        structure_table_length
      ),
    ));
  }

  log::trace!("Load SMBios structures from {}", smbioslib::DEV_MEM_FILE);
  let table = UndefinedStructTable::try_load_from_file_offset(
    &mut dev_mem,
    structure_table_addr,
    structure_table_length as usize,
  )?;

  Ok(SMBiosData::new(table, Some(version)))
}
