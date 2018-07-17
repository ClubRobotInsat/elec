#![deny(unsafe_code)]
#![no_main]
#![no_std]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate cortex_m;
#[macro_use]
extern crate librobot;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
#[macro_use(block)]
extern crate nb;
extern crate drs_0x01;
extern crate embedded_hal;
extern crate panic_semihosting;
extern crate stm32f446_hal;

use drs_0x01::*;

mod communicator;

use cortex_m::asm;
use embedded_hal::serial::{Read, Write};
use rt::ExceptionFrame;
use stm32f446_hal::prelude::*;
use stm32f446_hal::serial::Serial;
use stm32f446_hal::stm32f446;
use stm32f446_hal::stm32f446::USART3;

use stm32f446_hal::rcc::AHB1;

use librobot::arrayvec::ArrayVec;
use librobot::trame_reader::TrameReader;
use librobot::Trame;
//use communicator::Com;

entry!(main);

fn handle_trame(trame: Trame) -> Option<Trame> {
    match (
        trame.id,
        trame.cmd,
        &trame.data[0..trame.data_length as usize],
    ) {
        (0...5, 0x0, [0x55]) => Some(trame!(trame.id, 0x00, [0xAA])),
        (_, _, _) => None,
    }
}

fn main() -> ! {
    let p = stm32f446::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR;
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb1);

    // Configuration des horloges par défaut.
    let clocks = rcc.cfgr.max_speed(&mut flash.acr, &mut pwr); /*
        .cfgr
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .sysclk(128.mhz())
        .freeze(&mut flash.acr); */

    let tx = gpioc.pc10.into_af7(
        &mut gpioc.moder,
        &mut gpioc.afrh,
        &mut gpioc.otyper,
        &mut gpioc.pupdr,
    );

    let rx = gpioc.pc11.into_af7(
        &mut gpioc.moder,
        &mut gpioc.afrh,
        &mut gpioc.otyper,
        &mut gpioc.pupdr,
    );

    // Initialisation du périphérique USART
    //
    let serial = Serial::usart3(p.USART3, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb1);

    let (mut tx, mut rx) = serial.split();

    let m1 = MessageBuilder::new().id(0xFE).reboot().build();
    let m2 = MessageBuilder::new().id(0xFE).stat().build();

    for b in m1 {
        block!(tx.write(b)).unwrap();
    }

    loop {
        /*
        if let Ok(byte) = rx.read() {
            reader.step(byte);
        }
        if let Some(trame) = reader.pop_trame() {
            if let Some(sent) = handle_trame(trame) {
                let (arr, size): ([u8; 15], usize) = sent.into();
                for b in arr[0..size].iter() {
                    block!(tx.write(*b)).unwrap();
                }
            }
        }
        */
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
