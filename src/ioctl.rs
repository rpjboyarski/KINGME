use std::ffi::CString;
use nix::libc::{c_int, fclose, fileno, fopen, FS_IOC_GETFLAGS, FS_IOC_SETFLAGS, ioctl};
use crate::error::Error::IoctlError;
use crate::error::Result;

// All IOCTL calls are unsafe due to FFI
pub unsafe fn ioctl_file_get_flags(path: String) -> Result<i32> {
    let c_path = CString::new(path.clone())?;
    let c_read_flag = CString::new("r")?;

    let fp = fopen(c_path.as_ptr(), c_read_flag.as_ptr());

    let mut flag = 0;
    let status = ioctl(fileno(fp), FS_IOC_GETFLAGS, &flag);

    fclose(fp);

    if status != 0 {
        return Err(IoctlError(format!("Failed to set get flags for {path}")));
    }

    Ok(flag)
}

pub unsafe fn ioctl_file_set_flags(path: String, flag: c_int) -> Result<()> {
    let c_path = CString::new(path.clone())?;
    let c_read_flag = CString::new("r")?;

    let fp = fopen(c_path.as_ptr(), c_read_flag.as_ptr());

    let status = ioctl(fileno(fp), FS_IOC_SETFLAGS, &flag);

    fclose(fp);

    if status != 0 {
        return Err(IoctlError(format!("Failed to set immutable flag on {path}")));
    }

    Ok(())
}
