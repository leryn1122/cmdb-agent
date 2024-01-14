use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::OsString;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::marker::PhantomData;

use crate::support::kube::is_under_kubernetes;

const POSIX_HOST_NAME_MAX: libc::c_long = 255;
const KUBERNETES_NODE_NAME_KEY: &str = "K8S_NODE_NAME";

/// Get hostname
/// - In kubernetes, read environment variables, which is injected from `spec.nodeName` if in
///   kubernetes. In such case, it is not necessary to share the same UTS namespace with host.
/// - Otherwise, get hostname from syscall.
pub fn get_hostname() -> Result<String> {
  let hostname = if is_under_kubernetes() {
    std::env::var_os(KUBERNETES_NODE_NAME_KEY).ok_or(Error::from(ErrorKind::NotFound))
  } else {
    get_hostname_from_syscall()
  };

  hostname.map(|s| s.to_str().unwrap().to_string())
}

fn get_hostname_from_syscall() -> Result<OsString> {
  let limit = unsafe { libc::sysconf(libc::_SC_HOST_NAME_MAX) };
  let size = libc::c_long::max(limit, POSIX_HOST_NAME_MAX) as usize;

  let mut buff = vec![0u8; size + 1];
  let result = unsafe { libc::gethostname(buff.as_mut_ptr() as *mut libc::c_char, size) };

  if result != 0 {
    return Err(Error::last_os_error());
  }

  let end = buff.iter().position(|&b| b == b'\0').unwrap_or(buff.len());
  buff.resize(end, 0x00);
  Ok(unsafe { OsString::from_encoded_bytes_unchecked(buff) })
}

/// Read OS release info from `/etc/os-release`
pub fn get_os_release() -> Result<HashMap<String, String>> {
  let file = std::fs::OpenOptions::new().read(true).write(false).open("/etc/os-release")?;
  let reader = BufReader::new(file);
  let mut map = HashMap::new();

  for line in reader.lines() {
    let line = line?;
    let mut spilt = line.splitn(2, '=');
    map.insert(
      spilt.next().ok_or(Error::from(ErrorKind::InvalidData))?.to_string(),
      spilt.next().ok_or(Error::from(ErrorKind::InvalidData))?.replace("\n", ""),
    );
  }
  Ok(map)
}

#[derive(Debug)]
pub struct Uname {
  pub sysname:  String,
  pub nodename: String,
  pub release:  String,
  pub version:  String,
  pub machine:  String,
  _priv:        PhantomData<()>,
}

impl Uname {
  pub fn new() -> Result<Self> {
    let mut utsname = unsafe { std::mem::zeroed() };
    let res = unsafe { libc::uname(&mut utsname) };
    if res == 0 {
      Ok(From::from(utsname))
    } else {
      Err(Error::last_os_error())
    }
  }
}

#[inline]
fn to_cstr(buf: &[libc::c_char]) -> &CStr {
  unsafe { CStr::from_ptr(buf.as_ptr()) }
}

impl From<libc::utsname> for Uname {
  fn from(uts: libc::utsname) -> Self {
    Self {
      sysname:  to_cstr(&uts.sysname[..]).to_string_lossy().into_owned(),
      nodename: to_cstr(&uts.nodename[..]).to_string_lossy().into_owned(),
      release:  to_cstr(&uts.release[..]).to_string_lossy().into_owned(),
      version:  to_cstr(&uts.version[..]).to_string_lossy().into_owned(),
      machine:  to_cstr(&uts.machine[..]).to_string_lossy().into_owned(),
      _priv:    PhantomData,
    }
  }
}

pub fn uname() -> Result<Uname> {
  Uname::new()
}

#[cfg(test)]
mod tests {
  use std::process::Command;

  use super::*;

  #[test]
  fn test_get_hostname() {
    let expected = get_hostname().expect("Expected host name");
    let actual = String::from_utf8(Command::new("hostname").output().unwrap().stdout)
      .unwrap()
      .strip_suffix('\n')
      .unwrap()
      .to_string();
    assert!(expected.eq(&actual))
  }
}
