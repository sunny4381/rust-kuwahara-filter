use std::fmt;

use image;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ImageError(image::error::ImageError),
    FilterError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::IoError(ref err) => write!(f, "IO error: {}", err),
            Error::ImageError(ref err) => write!(f, "IO error: {}", err),
            Error::FilterError(ref err) => write!(f, "Filter error: {}", err),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<image::error::ImageError> for Error {
    fn from(err: image::error::ImageError) -> Error {
        Error::ImageError(err)
    }
}
