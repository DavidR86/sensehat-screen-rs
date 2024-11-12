//! Errors for the SenseHat Screen.
#[cfg(feature = "linux-framebuffer")]
use framebuffer::FramebufferError;
use std::fmt::Display;
#[cfg(feature = "fonts")]
use std::string::FromUtf16Error;

/// Errors that this crate can return
#[derive(Debug)]
pub enum ScreenError {
    #[cfg(feature = "linux-framebuffer")]
    Framebuffer(FramebufferError),
    #[cfg(feature = "fonts")]
    Unicode(FromUtf16Error),
}

impl Display for ScreenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "linux-framebuffer")]
            ScreenError::Framebuffer(err) => write!(f, "Framebuffer error: {}", err),
            #[cfg(feature = "fonts")]
            ScreenError::Unicode(err) => write!(f, "Unicode error: {}", err),
        }
    }
}

impl std::error::Error for ScreenError {}

#[cfg(feature = "linux-framebuffer")]
impl From<FramebufferError> for ScreenError {
    fn from(err: FramebufferError) -> ScreenError {
        ScreenError::Framebuffer(err)
    }
}

#[cfg(feature = "fonts")]
impl From<FromUtf16Error> for ScreenError {
    fn from(err: FromUtf16Error) -> ScreenError {
        ScreenError::Unicode(err)
    }
}
