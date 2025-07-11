mod files_lum;
mod loads_lum;

use thiserror::Error;

pub use self::files_lum::Error as FilesLumError;
pub use self::loads_lum::Error as LoadsLumError;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    LoadsLum(#[from] LoadsLumError),

    #[error(transparent)]
    FilesLum(#[from] FilesLumError),
}
