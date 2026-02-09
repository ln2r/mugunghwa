use argon2::password_hash::rand_core::{CryptoRng, Error as RandError, RngCore};
use getrandom::fill;
use std::num::NonZeroU32;

pub struct GetRandomWrapper;

impl CryptoRng for GetRandomWrapper {}

impl RngCore for GetRandomWrapper {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0u8; 4];
        fill(&mut buf).expect("getrandom failed");
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0u8; 8];
        fill(&mut buf).expect("getrandom failed");
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill(dest).expect("getrandom failed");
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), RandError> {
        fill(dest).map_err(|_| {
            // Any non-zero value is valid. Use 1.
            RandError::from(NonZeroU32::new(1).unwrap())
        })
    }
}
