use smbioslib::ProcessorFamily;
use smbioslib::ProcessorSpeed;
use smbioslib::SMBiosBaseboardInformation;
use smbioslib::SMBiosData;
use smbioslib::SMBiosInformation;
use smbioslib::SMBiosProcessorInformation;
use smbioslib::SMBiosSystemChassisInformation;
use smbioslib::SMBiosSystemInformation;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr;

use crate::support::smbios::error::BiosParseError;

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

// const BIOS_VENDOR: &str = "bios-vendor";
// const BIOS_VERSION: &str = "bios-version";
// const BIOS_RELEASE_DATE: &str = "bios-release-date";
// const BIOS_REVISION: &str = "bios-revision";
// const FIRMWARE_REVISION: &str = "firmware-revision";
// const SYSTEM_MANUFACTURER: &str = "system-manufacturer";
// const SYSTEM_PRODUCT_NAME: &str = "system-product-name";
// const SYSTEM_VERSION: &str = "system-version";
// const SYSTEM_SERIAL_NUMBER: &str = "system-serial-number";
// const SYSTEM_UUID: &str = "system-uuid";
// const SYSTEM_SKU_NUMBER: &str = "system-sku-number";
// const SYSTEM_FAMILY: &str = "system-family";
// const BASEBOARD_MANUFACTURER: &str = "baseboard-manufacturer";
// const BASEBOARD_PRODUCT_NAME: &str = "baseboard-product-name";
// const BASEBOARD_VERSION: &str = "baseboard-version";
// const BASEBOARD_SERIAL_NUMBER: &str = "baseboard-serial-number";
// const BASEBOARD_ASSET_TAG: &str = "baseboard-asset-tag";
// const CHASSIS_MANUFACTURER: &str = "chassis-manufacturer";
// const CHASSIS_TYPE: &str = "chassis-type";
// const CHASSIS_VERSION: &str = "chassis-version";
// const CHASSIS_SERIAL_NUMBER: &str = "chassis-serial-number";
// const CHASSIS_ASSET_TAG: &str = "chassis-asset-tag";
// const PROCESSOR_FAMILY: &str = "processor-family";
// const PROCESSOR_MANUFACTURER: &str = "processor-manufacturer";
// const PROCESSOR_VERSION: &str = "processor-version";
// const PROCESSOR_FREQUENCY: &str = "processor-frequency";

impl Keyword {
  pub fn parse(&self, data: &SMBiosData) -> Result<String, BiosParseError> {
    let func: fn(String, Option<String>) -> Option<String> = |mut acc, item| {
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
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::BaseboardManufacturerNotFound),
      Keyword::BaseboardProductName => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.product().to_utf8_lossy())
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::BaseboardProductNameNotFound),
      Keyword::BaseboardVersion => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.version().to_utf8_lossy())
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::BaseboardVersionNotFound),
      Keyword::BaseboardSerialNumber => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.serial_number().to_utf8_lossy())
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::BaseboardSerialNumberNotFound),
      Keyword::BaseboardAssetTag => data
        .map(|baseboard: SMBiosBaseboardInformation<'_>| baseboard.asset_tag().to_utf8_lossy())
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::BaseboardAssetTagNotFound),
      Keyword::ChassisManufacturer => data
        .map(|chassis_info: SMBiosSystemChassisInformation<'_>| {
          chassis_info.manufacturer().to_utf8_lossy()
        })
        .try_fold(String::new(), func)
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
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::ChassisVersionNotFound),
      Keyword::ChassisSerialNumber => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| chassis.serial_number().to_utf8_lossy())
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::ChassisSerialNumberNotFound),
      Keyword::ChassisAssetTag => data
        .map(|chassis: SMBiosSystemChassisInformation<'_>| {
          chassis.asset_tag_number().to_utf8_lossy()
        })
        .try_fold(String::new(), func)
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
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::ProcessorFamilyNotFound),
      Keyword::ProcessorManufacturer => data
        .map(|processor: SMBiosProcessorInformation<'_>| {
          processor.processor_manufacturer().to_utf8_lossy()
        })
        .try_fold(String::new(), func)
        .ok_or(BiosParseError::ProcessorManufacturerNotFound),
      Keyword::ProcessorVersion => data
        .map(|processor: SMBiosProcessorInformation<'_>| {
          processor.processor_version().to_utf8_lossy()
        })
        .try_fold(String::new(), func)
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
