use librobot::arrayvec::ArrayVec;
use librobot::trame_reader::TrameReader;
use librobot::Trame;

use stm32f446_hal::prelude::*;
use stm32f446_hal::serial::{Rx, RxPin, Serial, Tx, TxPin};

use nb::Error;
use nb::Error::WouldBlock;

use embedded_hal::serial::{Read, Write};

use stm32f446_hal::stm32f446::{USART1, USART2, USART3, USART6};

const BUFFER_SIZE: usize = 1024;
/* PAS LA BONNE FACON DE FAIRE LES CHOSES
pub struct Com<USART> {
    tx: Tx<USART>,
    rx: Rx<USART>,
    reader: TrameReader,
    trames_to_send: ArrayVec<[Trame; BUFFER_SIZE]>,
}

impl<USART> Com<USART> where
    USART {
    pub fn new<TX, RX>(ser: ::stm32f446_hal::serial::Serial<USART, (TX, RX)>) -> Self
    where
        TX: TxPin<USART>,
        RX: RxPin<USART>,
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

macro_rules! impl_com {
    ($USARTX:ident) => {
        impl Com<$USARTX> {
            pub fn new<TX, RX>(ser: ::stm32f446_hal::serial::Serial<$USARTX, (TX, RX)>) -> Self
            where
                TX: TxPin<$USARTX>,
                RX: RxPin<$USARTX>,
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
    };
}
*/
