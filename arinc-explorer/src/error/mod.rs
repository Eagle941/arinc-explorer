mod loads_lum;

use thiserror::Error;

pub use self::loads_lum::Error as LoadsLumError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LoadsLum(#[from] LoadsLumError),
}
