// interrupts.rs
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // your main function code here
    loop {}
}

// Define the interrupt handlers
#[allow(unused)]
fn DefaultHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn NMIHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn HardFaultHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn MemManageHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn BusFaultHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn UsageFaultHandler_() -> ! {
    loop {}
}

#[allow(unused)]
fn SVC_Handler_() -> ! {
    loop {}
}

#[allow(unused)]
fn DebugMon_Handler_() -> ! {
    loop {}
}

#[allow(unused)]
fn PendSV_Handler_() -> ! {
    loop {}
}

#[allow(unused)]
fn SysTick_Handler_() -> ! {
    loop {}
}