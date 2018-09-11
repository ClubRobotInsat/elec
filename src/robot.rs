use stm32f446_hal::stm32f446;
use stm32f446_hal::stm32f446::Interrupt;
use stm32f446_hal::stm32f446::{USART3, USART6};

use stm32f446_hal::delay::Delay;
use stm32f446_hal::prelude::*;
use stm32f446_hal::rcc::AHB1;
use stm32f446_hal::serial::Serial;
use stm32f446_hal::serial::{Rx, Tx};

use cortex_m::peripheral::NVIC;
use cortex_m::Peripherals as CortexPeripherals;

/// Contiens toutes les structures représentant l'interface avec le µ-controlleur
pub struct Robot {
    pub servo_rx: Rx<USART3>,
    pub servo_tx: Tx<USART3>,
    pub pc_rx: Rx<USART6>,
    pub pc_tx: Tx<USART6>,
    pub delay: Delay,
    pub nvic: NVIC,
}

/// Crée un robot en initialisant tous les périphériques
pub fn init_peripherals(p: stm32f446::Peripherals, k: CortexPeripherals) -> Robot {
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR;
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb1);
    let mut nvic = k.NVIC;

    nvic.enable(Interrupt::USART6);

    // Configuration des horloges par défaut.
    let clocks = rcc.cfgr.max_speed(&mut flash.acr, &mut pwr); /*
        .cfgr
        .pclk1(45.mhz())
        .pclk2(90.mhz())
        .sysclk(128.mhz())
        .freeze(&mut flash.acr); */

    let delay = Delay::new(k.SYST, clocks);

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

    let (servo_tx, servo_rx) = serial.split();

    let tx = gpioc.pc6.into_af7(
        &mut gpioc.moder,
        &mut gpioc.afrl,
        &mut gpioc.otyper,
        &mut gpioc.pupdr,
    );

    let rx = gpioc.pc7.into_af7(
        &mut gpioc.moder,
        &mut gpioc.afrl,
        &mut gpioc.otyper,
        &mut gpioc.pupdr,
    );

    let serial = Serial::usart6(p.USART6, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);

    let (pc_tx, pc_rx) = serial.split();

    Robot {
        servo_rx: servo_rx,
        servo_tx: servo_tx,
        pc_tx: pc_tx,
        pc_rx: pc_rx,
        delay: delay,
        nvic: nvic,
    }
}
