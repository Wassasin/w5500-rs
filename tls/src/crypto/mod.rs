mod aes;
mod record;

pub use aes::Aes128Gcm;
pub use record::{decrypt_record_inplace, encrypt_record_inplace};
