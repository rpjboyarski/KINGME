use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    IoctlError(String),
    // External
    #[from]
    NixError(nix::Error),
    #[from]
    NulError(std::ffi::NulError),
    #[from]
    StdIoError(std::io::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
