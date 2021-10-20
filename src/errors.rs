#![allow(missing_docs)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustNFTError {
    #[error("unknown Terra-Rust NFT error")]
    _Unknown,
}
