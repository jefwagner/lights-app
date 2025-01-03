use std::ops::{Index, IndexMut};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};
use anyhow::Result;

use super::{DriverConfig, LedColor, LedIterator};

/// Driver object for the ws281x lights
pub struct LedDriver {
    pub conf: DriverConfig,
    pub leds: Vec<[u8; 4]>,
}

impl LedDriver {
    /// new driver
    pub fn new(conf: DriverConfig) -> Result<Self> {
        let size = conf.left + conf.right;
        let mut leds = Vec::with_capacity(size);
        for _ in 0..size {
            leds.push([0,0,0,0]);
        }
        Ok(LedDriver { conf, leds })
    }

    /// The number of LEDs controlled by the driver
    pub fn size(&self) -> usize {
        self.leds.len()
    }

    /// Create a mutable iterator for the LEDS
    pub fn iter(&mut self) -> LedIterator {
        LedIterator{ lc: self, index: 0 }
    }

    /// Turn all the lights off
    pub fn clear(&mut self) -> Result<()> {
        for led in self.iter() {
            *led = [0,0,0,0];
        }
        Ok(())
    }

    /// Fill all the lights with a solid color
    pub fn fill<A: Copy + Into<LedColor>>(&mut self, color: A) -> Result<()> {
        let color: [u8; 4] = color.into().into();
        for led in self.iter() {
            *led = color;
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Index<usize> for LedDriver {
    type Output = [u8; 4];

    /// Get the reference to a single LED
    fn index(&self, index: usize) -> &Self::Output {
        &self.leds[index]
    }
}

impl IndexMut<usize> for LedDriver {

    /// Get a mutable reference to a single LED
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.leds[index]
    }
}

impl<'a> Iterator for LedIterator<'a> {
    type Item = &'a mut [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        if i < self.lc.leds.len() {
            let ptr = self.lc.leds.as_mut_ptr();
            unsafe {
                Some(&mut *ptr.add(i))
            }
        } else {
            None
        }
    }
}
