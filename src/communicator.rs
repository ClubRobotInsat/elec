use librobot::arrayvec::ArrayVec;
use librobot::trame_reader::TrameReader;
use librobot::Trame;

use stm32f446_hal::prelude::*;
use stm32f446_hal::serial::{Rx, RxPin, Serial, Tx, TxPin};

use nb::Error;
use nb::Error::WouldBlock;

use embedded_hal::serial::{Read, Write};

use stm32f446_hal::stm32f446::USART1;

const BUFFER_SIZE: usize = 1024;

pub struct Com<USART> {
    tx: Tx<USART>,
    rx: Rx<USART>,
    reader: TrameReader,
    trames_to_send: ArrayVec<[Trame; BUFFER_SIZE]>,
}

impl Com<USART1> {
    pub fn new<TX, RX>(ser: ::stm32f446_hal::serial::Serial<USART1, (TX, RX)>) -> Self
    where
        TX: TxPin<USART1>,
        RX: RxPin<USART1>,
    {
        let (tx, rx) = ser.split();
        Com {
            tx,
            rx,
            reader: TrameReader::new(),
            trames_to_send: ArrayVec::new(),
        }
    }

    pub fn read(&mut self) {
        while let Ok(data) = self.rx.read() {
            self.reader.parse(&[data]);
        }
    }

    pub fn get_trame(&mut self) -> Result<Trame, Error<()>> {
        match self.reader.pop_trame() {
            Some(t) => Ok(t),
            None => Err(WouldBlock),
        }
    }

    pub fn send_trame(&mut self, _t: Trame) {
        unimplemented!()
    }
}
