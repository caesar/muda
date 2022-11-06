use thiserror::Error;

/// Errors returned by muda.
#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("This menu item is not a child of this `Menu` or `Submenu`")]
    NotAChildOfThisMenu,
    #[cfg(windows)]
    #[error("This menu has not been initialized for this hwnd`")]
    NotInitialized,
    #[cfg(target_os = "linux")]
    #[error("This menu has not been initialized for this gtk window`")]
    NotInitialized,
    #[error("{0}")]
    AcceleratorParseError(String),
    #[error("Cannot map {0} to gdk key")]
    AcceleratorKeyNotSupported(keyboard_types::Code),
}

/// Convenient type alias of Result type for muda.
pub type Result<T> = std::result::Result<T, Error>;
