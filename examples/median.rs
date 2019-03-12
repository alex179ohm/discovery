#![feature(asm)]
#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::{iprintln, Peripherals, peripheral::ITM};
use cortex_m_rt::entry;
//use cortex_m_semihosting::{debug, hprintln};

fn aux() -> ITM  {
    let p = Peripherals::take().unwrap();
    p.ITM
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn midpoint(a: Point, b: Point) -> Point {
    let m_x: u32;
    let m_y: u32;

    unsafe {
        asm!("add $0, $2, $4
              add $1, $3, $5
              asr $0, $0, #1
              asr $1, $1, #1
            "
            : "=&r"(m_x), "=&r"(m_y)
            : "r"(a.x), "r"(a.y), "r"(b.x), "r"(b.y) ::);
    }
    Point { x: m_x, y: m_y }
}

#[entry]
fn main() -> ! {
    //let mut p = Peripherals::take().unwrap();
    //let stim = &mut p.ITM.stim[0];
    let a = Point { x: 5, y: 6 };
    let b = Point { x: 15, y: 27 };
    let m = midpoint(a, b);
    let mut aux = aux();
    iprintln!(&mut aux.stim[0], "a: {:?}, b: {:?}: midpoint: {:?}", a, b, m);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
