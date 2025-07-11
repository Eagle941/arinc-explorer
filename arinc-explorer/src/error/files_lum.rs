use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    FileIO(#[from] std::io::Error),

    #[error(transparent)]
    BinRead(#[from] binrw::Error),
}
