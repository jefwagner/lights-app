use std::ops::{Index, IndexMut};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use anyhow::{Context, Result};
use rs_ws281x::{
    ChannelBuilder,
    Controller,
    ControllerBuilder,
    StripType,
};

use super::LedColor;

/// Configuration for the lights
#[derive(Copy, Clone)]
pub struct DriverConfig {
    /// number of lights in the left strip (GPIO 12 - Pin 32)
    pub left: usize,
    /// number of lights in the right strip (GPIO 13 - Pin 33)
    pub right: usize,
    /// maybe max overall brightness?
    pub brightness: u8,
}

/// Driver object for the ws281x lights
pub struct LedDriver {
    pub sizes: (usize, usize),
    pub controller: Controller,
}

impl LedDriver {
    /// new driver
    pub fn new(config: DriverConfig) -> Result<Self> {
        debug!("Creating new LED driver with ({},{}) leds on (left,right) pins", config.left, config.right);
        let controller = ControllerBuilder::new()
        .channel(
            0,
            ChannelBuilder::new()
            .pin(12) // GPIO 12 = header pin 32
            .count(config.left as i32)
            .strip_type(StripType::Ws2812)
            .brightness(config.brightness)
            .build(),
        )
        .channel(
            1,
            ChannelBuilder::new()
            .pin(13) // GPIO = header pin 33
            .count(config.right as i32)
            .strip_type(StripType::Ws2812)
            .brightness(config.brightness)
            .build(),
        )
        .build()
        .context("Failed setting up Controller")?;

        Ok(LedDriver {
            sizes: (config.left, config.right),
            controller,
        })
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
        Ok(self.controller.render()?)
    }

    /// Fill all the lights with a solid color
    pub fn fill<A: Copy + Into<LedColor>>(&mut self, color: A) -> Result<()> {
        let color: [u8; 4] = color.into().into();
        for led in self.iter() {
            *led = color;
        }
        Ok(self.controller.render()?)
    }
}

impl Index<usize> for LedDriver {
    type Output = [u8; 4];

    /// Get the reference to a single LED
    fn index(&self, index: usize) -> &Self::Output {
        let mid = self.sizes.0;
        if index < mid {
            &self.controller.leds(0)[mid-1-index]
        } else {
            &self.controller.leds(1)[index-mid]
        }
    }
}

impl IndexMut<usize> for LedDriver {

    /// Get a mutable reference to a single LED
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mid = self.sizes.0;
        if index < mid {
            &mut self.controller.leds_mut(0)[mid-1-index-1]
        } else {
            &mut self.controller.leds_mut(1)[index-mid-1]
        }
    }
}

/// Iterator object for the Lights Driver
pub struct LedIterator<'a> {
    lc: &'a mut LedDriver,
    index: usize,
}

impl<'a> Iterator for LedIterator<'a> {
    type Item = &'a mut [u8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        let i = self.index;
        self.index += 1;
        let mid = self.lc.sizes.0;
        let last = mid + self.lc.sizes.1;
        if i < mid {
            let ptr = self.lc.controller.leds_mut(0).as_mut_ptr();
            unsafe {
                Some(&mut *ptr.add(mid-1-i))
            }
        } else if i < last {
            let ptr = self.lc.controller.leds_mut(1).as_mut_ptr();
            unsafe {
                Some(&mut *ptr.add(i-mid))
            }
        } else {
            None
        }
    }
}
