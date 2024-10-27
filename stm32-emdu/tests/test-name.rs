#![no_std]
#![no_main]

extern crate defmt_test;

use panic_halt as _; 
use cortex_m as _;
use  cortex_m_rt as _;
// Panic handler for the test


// Define the state structure used in tests
struct MyState {
    flag: bool,
}

#[defmt_test::tests]
mod tests {
    use super::*;

    #[init]
    fn init() -> MyState {
        MyState {
            flag: true,
        }
    }

    #[before_each]
    fn before_each(state: &mut MyState) {
        defmt::println!("State flag before is {}", state.flag);
    }

    #[after_each]
    fn after_each(state: &mut MyState) {
        defmt::println!("State flag after is {}", state.flag);
    }

    #[test]
    fn assert_true() {
        assert!(true);
    }

    #[test]
    fn assert_flag(state: &mut MyState) {
        assert!(state.flag);
        state.flag = false;
    }
}
