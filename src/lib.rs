#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
pub mod i2cdev;
pub mod platform;
