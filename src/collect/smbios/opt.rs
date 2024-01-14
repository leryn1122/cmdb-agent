use std::collections::HashSet;
use std::str::FromStr;

use smbioslib::ProcessorFamily;
use smbioslib::ProcessorSpeed;
use smbioslib::SMBiosBaseboardInformation;
use smbioslib::SMBiosData;
use smbioslib::SMBiosInformation;
use smbioslib::SMBiosProcessorInformation;
use smbioslib::SMBiosStruct;
use smbioslib::SMBiosSystemChassisInformation;
use smbioslib::SMBiosSystemInformation;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

use crate::collect::smbios::error::BiosParseError;

#[derive(Debug, EnumString, EnumIter, IntoStaticStr)]
pub enum Keyword {
  BiosVendor,
  BiosVersion,
  BiosReleaseDate,
  BiosRevision,
  FirmwareRevision,
  SystemManufacturer,
  SystemProductName,
  SystemVersion,
  SystemSerialNumber,
  SystemUuid,
  SystemSkuNumber,
  SystemFamily,
  BaseboardManufacturer,
  BaseboardProductName,
  BaseboardVersion,
  BaseboardSerialNumber,
  BaseboardAssetTag,
  ChassisManufacturer,
  ChassisType,
  ChassisVersion,
  ChassisSerialNumber,
  ChassisAssetTag,
  ProcessorFamily,
  ProcessorManufacturer,
  ProcessorVersion,
  ProcessorFrequency,
}

impl Keyword {
  pub fn parse(&self, data: &SMBiosData) -> Result<String, BiosParseError> {
    let concat_functor: fn(String, Option<String>) -> Option<String> = |mut acc, item| {
      item.map(|val| {
        if !acc.is_empty() {
          acc.push('\n');
        }
        acc.push_str(&val);
        acc
      })
    };

    match self {
      Keyword::BiosVendor => data
        .find_map(|bios: SMBiosInformation<'_>| bios.vendor().to_utf8_lossy())
        .ok_or(BiosParseError::BiosVendorNotFound),
      Keyword::BiosVersion => data
        .find_map(|bios: SMBiosInformation<'_>| bios.version().to_utf8_lossy())
        .ok_or(BiosParseError::BiosVersionNotFound),
      Keyword::BiosReleaseDate => data
        .find_map(|bios: SMBiosInformation<'_>| bios.release_date().to_utf8_lossy())
        .ok_or(BiosParseError::BiosReleaseDateNotFound),
      Keyword::BiosRevision => data
        .find_map(|bios: SMBiosInformation<'_>| {
          match (
            bios.system_bios_major_release(),
            bios.system_bios_minor_release(),
          ) {
            (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
            _ => None,
          }
        })
        .ok_or(BiosParseError::BiosRevisionNotFound),
      Keyword::FirmwareRevision => data
        .find_map(|bios: SMBiosInformation<'_>| {
          match (
            bios.e_c_firmware_major_release(),
            bios.e_c_firmware_minor_release(),
          ) {
            (Some(major), Some(minor)) => Some(format!("{}.{}", major, minor)),
            _ => None,
          }
        })
        .ok_or(BiosParseError::FirmwareRevisionNotFound),
      Keyword::SystemManufacturer => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.manufacturer().to_utf8_lossy())
        .ok_or(BiosParseError::SystemManufacturerNotFound),
      Keyword::SystemProductName => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.product_name().to_utf8_lossy())
        .ok_or(BiosParseError::SystemProductNameNotFound),
      Keyword::SystemVersion => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.version().to_utf8_lossy())
        .ok_or(BiosParseError::SystemVersionNotFound),
      Keyword::SystemSerialNumber => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.serial_number().to_utf8_lossy())
        .ok_or(BiosParseError::SystemSerialNumberNotFound),
      Keyword::SystemUuid => {
        match data.find_map(|system: SMBiosSystemInformation<'_>| system.uuid()) {
          // SystemUuidData is an enum that can be broken down further if desired
          Some(uuid) => Ok(format!("{}", uuid)),
          None => Err(BiosParseError::SystemUuidNotFound),
        }
      }
      Keyword::SystemSkuNumber => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.sku_number().to_utf8_lossy())
        .ok_or(BiosParseError::SystemSkuNumberNotFound),
      Keyword::SystemFamily => data
        .find_map(|system: SMBiosSystemInformation<'_>| system.family().to_utf8_lossy())
        .ok_or(BiosParseError::SystemFamilyNotFound),
      Keyword::BaseboardManufacturer => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.manufacturer().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::BaseboardManufacturerNotFound),
      Keyword::BaseboardProductName => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.product().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::BaseboardProductNameNotFound),
      Keyword::BaseboardVersion => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.version().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::BaseboardVersionNotFound),
      Keyword::BaseboardSerialNumber => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.serial_number().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
      Keyword::BaseboardAssetTag => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.asset_tag().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::BaseboardAssetTagNotFound),
      Keyword::ChassisManufacturer => data
        .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
          chassis_info.manufacturer().to_utf8_lossy()
        })
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ChassisManufacturerNotFound),
      Keyword::ChassisType => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| chassis.chassis_type())
        .try_fold(String::new(), |mut acc, item| {
          item.map(|val| {
            if !acc.is_empty() {
              acc.push('\n');
            };
            acc.push_str(&format!("{}", &val).to_string());
            acc
          })
        })
        .ok_or(BiosParseError::ChassisTypeNotFound),
      Keyword::ChassisVersion => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| chassis.version().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ChassisVersionNotFound),
      Keyword::ChassisSerialNumber => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| chassis.serial_number().to_utf8_lossy())
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ChassisSerialNumberNotFound),
      Keyword::ChassisAssetTag => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| {
          chassis.asset_tag_number().to_utf8_lossy()
        })
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ChassisAssetTagNotFound),
      Keyword::ProcessorFamily => data
        .map(|processor: SMBiosProcessorInformation<'_>| {
          if let Some(family) = processor.processor_family() {
            match family.value {
              ProcessorFamily::SeeProcessorFamily2 => {
                processor.processor_family_2().map(|family2| format!("{}", family2))
              }
              _ => Some(format!("{}", family)),
            }
          } else {
            None
          }
        })
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ProcessorFamilyNotFound),
      Keyword::ProcessorManufacturer => data
        .map(|processor: SMBiosProcessorInformation<'_>| {
          processor.processor_manufacturer().to_utf8_lossy()
        })
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ProcessorManufacturerNotFound),
      Keyword::ProcessorVersion => data
        .map(|processor: SMBiosProcessorInformation<'_>| {
          processor.processor_version().to_utf8_lossy()
        })
        .try_fold(String::new(), concat_functor)
        .ok_or(BiosParseError::ProcessorVersionNotFound),
      Keyword::ProcessorFrequency => data
        .map(|processor: SMBiosProcessorInformation<'_>| processor.current_speed())
        .try_fold(String::new(), |mut acc, item| {
          item.map(|val| {
            if !acc.is_empty() {
              acc.push('\n');
            };
            let output = match &val {
              ProcessorSpeed::Unknown => String::from("Unknown"),
              ProcessorSpeed::MHz(frequency) => format!("{} MHz", frequency),
            };
            acc.push_str(output.as_str());
            acc
          })
        })
        .ok_or(BiosParseError::ProcessorFrequencyNotFound),
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum BiosType {
  Bios,
  System,
  Baseboard,
  Chassis,
  Processor,
  Memory,
  Cache,
  Connector,
  Slot,
  Numeric(u8),
}

impl BiosType {
  #[allow(unused)]
  pub fn parse<'a, T>(bios_type: &BiosType, data: &'a SMBiosData) -> T
  where
    T: SMBiosStruct<'a>,
  {
    let bios_types: HashSet<u8> = bios_type.into_iter().collect();

    let undefined_struct = data
      .iter()
      .find(|undefined_struct| bios_types.contains(&undefined_struct.header.struct_type()))
      .unwrap();
    T::new(undefined_struct)
  }

  #[allow(unused)]
  pub fn parse_vec<'a, T>(bios_type: &BiosType, data: &'a SMBiosData) -> Vec<T>
  where
    T: SMBiosStruct<'a>,
  {
    let bios_types: HashSet<u8> = bios_type.into_iter().collect();

    data
      .iter()
      .filter(|u| bios_types.contains(&u.header.struct_type()))
      .map(|s| T::new(s))
      .collect()
  }
}

impl From<BiosType> for HashSet<u8> {
  fn from(val: BiosType) -> Self {
    val.into_iter().collect()
  }
}

impl FromStr for BiosType {
  type Err = std::num::ParseIntError;

  #[rustfmt::skip]
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "bios"        => Ok(BiosType::Bios),
      "system"      => Ok(BiosType::System),
      "baseboard"   => Ok(BiosType::Baseboard),
      "chassis"     => Ok(BiosType::Chassis),
      "processor"   => Ok(BiosType::Processor),
      "memory"      => Ok(BiosType::Memory),
      "cache"       => Ok(BiosType::Cache),
      "connector"   => Ok(BiosType::Connector),
      "slot"        => Ok(BiosType::Slot),
      _ => Ok(BiosType::Numeric(u8::from_str(s)?)),
    }
  }
}

impl IntoIterator for BiosType {
  type IntoIter = std::vec::IntoIter<Self::Item>;
  type Item = u8;

  /// Keyword     Types
  /// ------------------------------------
  /// bios        0, 13
  /// system      1, 12, 15, 23, 32
  /// baseboard   2, 10, 41
  /// chassis     3
  /// processor   4
  /// memory      5, 6, 16, 17
  /// cache       7
  /// connector   8
  /// slot        9
  #[rustfmt::skip]
  fn into_iter(self) -> Self::IntoIter {
    match self {
      BiosType::Bios      => vec![0, 13].into_iter(),
      BiosType::System    => vec![1, 12, 15, 23, 32].into_iter(),
      BiosType::Baseboard => vec![2, 10, 41].into_iter(),
      BiosType::Chassis   => vec![3].into_iter(),
      BiosType::Processor => vec![4].into_iter(),
      BiosType::Memory    => vec![5, 6, 16, 17].into_iter(),
      BiosType::Cache     => vec![7].into_iter(),
      BiosType::Connector => vec![8].into_iter(),
      BiosType::Slot      => vec![9].into_iter(),
      BiosType::Numeric(number) => vec![number].into_iter(),
    }
  }
}
