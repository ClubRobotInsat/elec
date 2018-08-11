use drs_0x01::prelude::*;

use core::ops::Deref;

pub struct ServoManager {
    servo: [Servo; 255],
}

impl ServoManager {
    /// Crée un nouveau servomanager
    pub fn new() -> ServoManager {
        let mut servo: [Servo; 255] = [Default::default(); 255];
        for (i, s) in servo.iter_mut().enumerate() {
            s.set_id(i as u8);
        }
        ServoManager { servo }
    }
}

impl Deref for ServoManager {
    type Target = [Servo];
    fn deref(&self) -> &Self::Target {
        &self.servo
    }
}
