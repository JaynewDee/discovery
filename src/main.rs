#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;

    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();

    while !systick.has_wrapped() {
        print_hello();
    }

    loop {
        hprintln!("Hit main loop");
    }
}

fn print_hello() {
    hprintln!("Hellooooo").unwrap();
}
