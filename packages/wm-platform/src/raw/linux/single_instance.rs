use std::ffi::{CStr, CString};

use anyhow::{bail, Context};

use crate::CommonSingleInstance;

const SEM_NAME: &str = "/glazewm_single_instance";

pub struct SingleInstance(*mut libc::sem_t);

impl CommonSingleInstance for SingleInstance {
  fn new() -> anyhow::Result<Self> {
    let c_str =
      CString::new(SEM_NAME).context("Failed to create CString")?;
    let sem = unsafe {
      libc::sem_open(
        c_str.as_ptr(),
        libc::O_CREAT | libc::O_EXCL,
        libc::O_RDONLY,
        1,
      )
    };

    if sem == libc::SEM_FAILED {
      // Saftey: `raw_os_error` will always be some when called on
      // `last_os_error`
      let err = std::io::Error::last_os_error().raw_os_error().unwrap();
      if err == libc::EEXIST {
        bail!("Another instance of the application is already running.");
      }
      return Err(anyhow::anyhow!(
        "Failed to create semaphore: {}",
        std::io::Error::from_raw_os_error(err)
      ));
    }

    Ok(SingleInstance(sem))
  }

  fn is_running() -> bool {
    let c_str = CString::new(SEM_NAME).expect("Failed to create CString");
    let sem = unsafe {
      libc::sem_open(
        c_str.as_ptr(),
        libc::O_CREAT | libc::O_EXCL,
        libc::O_RDONLY,
        1,
      )
    };

    if sem == libc::SEM_FAILED {
      // Saftey: `raw_os_error` will always be some when called on
      // `last_os_error`
      let err = std::io::Error::last_os_error().raw_os_error().unwrap();
      if err == libc::EEXIST {
        return true; // Another instance is running
      }
    }

    false
  }
}

impl Drop for SingleInstance {
  fn drop(&mut self) {
    let c_str = CString::new(SEM_NAME)
      .expect("Failed to create CString for SEM_NAME");
    unsafe {
      let _ = libc::sem_close(self.0);
      let _ = libc::sem_unlink(c_str.as_ptr());
    }
  }
}
