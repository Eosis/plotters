#[cfg(test)]
mod mocked;
#[cfg(test)]
pub use mocked::{create_mocked_drawing_area, MockedBackend, check_color};

/// This is the dummy backend placeholder for the backend that never fails
#[derive(Debug)]
pub struct DummyBackendError;

impl std::fmt::Display for DummyBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for DummyBackendError {}
