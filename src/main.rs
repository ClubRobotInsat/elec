#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
#[macro_use(block)]
extern crate nb;
extern crate panic_semihosting;
extern crate stm32f446_hal;
extern crate embedded_hal;

use cortex_m::asm;
use stm32f446_hal::stm32f446;
use stm32f446_hal::prelude::*;
use stm32f446_hal::serial::Serial;
use embedded_hal::serial::{Read,Write};
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32f446::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb1);
    let mut gpiob = p.GPIOB.split(&mut rcc.ahb1);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    // let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    // let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpiob.pb7.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    // TRY using a different USART peripheral here
    let serial = Serial::usart1(p.USART1, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb2);
    let (mut tx, mut rx) = serial.split();

    let sent = b'X';

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`
    block!(tx.write(sent)).ok();

    let received = block!(rx.read()).unwrap();

    assert_eq!(received, sent);

    // if all goes well you should reach this breakpoint
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
