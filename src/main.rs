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
#[macro_use]
extern crate stm32f446_hal;

use drs_0x01::prelude::*;

mod communicator;
mod robot;
mod servo;

use core::cell::RefCell;

use robot::{init_peripherals, Robot};

use cortex_m::asm;
use embedded_hal::serial::{Read, Write};
use rt::ExceptionFrame;
use stm32f446_hal::prelude::*;
use stm32f446_hal::stm32f446;

use librobot::arrayvec::ArrayVec;
use librobot::trame_reader::TrameReader;
use librobot::Trame;

use drs_0x01::addr::WritableRamAddr;
use servo::ServoManager;

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

/// Envoie 3 messages d'initialisation aux servomoteurs :
/// * Reboot
/// * Toujours renvoyer des ack (pour le debug)
/// * Activer le couple
fn init_servo(robot: &mut Robot) {
    let servos = ServoManager::new();

    let m2 = servos[0xFE].reboot();

    for b in m2 {
        block!(robot.servo_tx.write(b)).unwrap();
    }

    for _ in 0..5 {
        robot.delay.delay_ms(70 as u32);
    }

    let m2 = servos[0xFE].ram_write(WritableRamAddr::AckPolicy(2));

    for b in m2 {
        block!(robot.servo_tx.write(b)).unwrap();
    }

    let m1 = servos[0xFE].enable_torque();

    for b in m1 {
        block!(robot.servo_tx.write(b)).unwrap();
    }
}

fn main() -> ! {
    let mut robot = init_peripherals(
        stm32f446::Peripherals::take().unwrap(),
        cortex_m::Peripherals::take().unwrap(),
    );

    init_servo(&mut robot);

    let mut reader = TrameReader::new();

    loop {
        let b = block!(robot.pc_rx.read()).unwrap();
        reader.step(b);
        if let Some(trame) = reader.pop_trame() {
            asm::bkpt();
        }

        /*
        let mess = servos[0x05].stat();
        for b in mess {
            block!(robot.servo_tx.write(b)).unwrap();
        }

        robot.delay.delay_ms(70 as u16);
*/
        /*
        if let Ok(byte) = pc_rx.read() {
            reader.step(byte);
        }
        if let Some(trame) = reader.pop_trame() {
            if let Some(sent) = handle_trame(trame) {
                let (arr, size): ([u8; 15], usize) = sent.into();
                for b in arr[0..size].iter() {
                    block!(pc_tx.write(*b)).unwrap();
                }
            }
        }*/
    }
}

interrupt!(USART6, usart_pc);

fn usart_pc() {}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("Hardfault... : {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
