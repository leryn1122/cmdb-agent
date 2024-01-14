use getset::CopyGetters;
use getset::Getters;
use getset::MutGetters;
use getset::Setters;
use serde::Deserialize;
use serde::Serialize;

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct MachineInfo {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  hostname:      String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  serial_number: String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  cloud:         Option<String>,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  os:            OS,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  devices:       Devices,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  networks:      Vec<Network>,
}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct Cloud {}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct OS {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  platform: String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  os:       String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  version:  String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  arch:     String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  kernel:   String,
}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct Devices {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  processor: Vec<Processor>,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  memory:    Memory,
}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct Processor {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  version:        String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  max_speed:      u16,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  current_speed:  u16,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  core_count:     u16,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  cores_enabled:  u16,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  thread_count:   u16,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  thread_enabled: u16,
}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  total_memory: u64,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  unit:         String,
}

#[derive(
  Clone, Debug, Default, Serialize, Deserialize, Getters, Setters, MutGetters, CopyGetters,
)]
#[serde(rename_all = "camelCase")]
pub struct Network {
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  name:        String,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  address:     Vec<String>,
  #[getset(get = "pub", set = "pub", get_mut = "pub")]
  mac_address: String,
}
