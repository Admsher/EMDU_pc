
#![no_std]
#![no_main]
// use core::prelude::rust_2024::test;

use core::assert_eq;
use panic_halt as _;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_something() {
        assert_eq!(stm32_example::main(), true);
    }
}