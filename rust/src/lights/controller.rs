use std::{iter::zip, future::Future};
use anyhow::Result;
use tokio::sync::mpsc;
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use super::{LedColor, LedDriver, DriverConfig};

/// Commands to send for the lights
#[derive(Clone)]
pub enum LightsCommand {
    /// Stop the system completely
    Stop,
    /// Turn all the lights off
    Off,
    /// Turn the lights back on in last state?
    On,
    /// Turn all the lights onto a single color
    Fill(LedColor),
    /// Set the color of a single LED
    SetSingle(usize, LedColor),
    /// Set the color of all LEDs
    Set(Vec<LedColor>),
    /// Change the configuration for the driver
    ChangeConfig(DriverConfig),
}

#[derive(Clone)]
pub struct LightsRemote {
    sender: mpsc::Sender<LightsCommand>,
}

impl LightsRemote {
    pub fn new(sender: mpsc::Sender<LightsCommand>) -> Self {
        LightsRemote{ sender }
    }

    pub async fn send(&self, cmd: LightsCommand) -> Result<()> {
        Ok(self.sender.send(cmd).await?)
    }
}

pub trait LightsController {
    fn new(config: DriverConfig, receiver: mpsc::Receiver<LightsCommand>) -> impl LightsController;
    fn start(&mut self) -> impl Future<Output = Result<()>>;        
}

pub struct RealLightsController {
    config: DriverConfig,
    receiver: mpsc::Receiver<LightsCommand>,
    state: Vec<LedColor>,
}

impl LightsController for RealLightsController {
    fn new(config: DriverConfig, receiver: mpsc::Receiver<LightsCommand>) -> impl LightsController {
        let mut default_colors: Vec<LedColor> = Vec::with_capacity(config.left + config.right);
        let red: LedColor = [128, 0, 0].into();
        let green: LedColor = [0, 128, 0].into();
        let white: LedColor = [96, 96, 64].into();
        for i in 0..(config.left + config.right) {
            if i%3 == 0 {
                default_colors.push(red);
            } else if i%3 == 1 {
                default_colors.push(green);
            } else if i%3 == 2 {
                default_colors.push(white);
            }
        }
        // for i in 0..(config.left + config.right) {
        //     if i < config.left {
        //         default_colors.push(red);
        //     } else {
        //         default_colors.push(green);
        //     }
        // }
        RealLightsController { config, receiver, state: default_colors }
    }

    /// Consume the controller and start a loop to control the lights based on MPSC messages
    async fn start(&mut self) -> Result<()> {
        debug!("Starting lights controller");
        let mut driver = LedDriver::new(self.config)?;
        for (state_led, led) in zip(self.state.iter(), driver.iter()) {
            *led = (*state_led).into()
        }
        driver.controller.render()?;
        while let Some(cmd) = self.receiver.recv().await {
            match cmd {
                LightsCommand::Off => {
                    trace!("Turning lights off");
                    driver.clear()?;
                },
                LightsCommand::On => {
                    trace!("Turingin lights on");
                    for (state_led, led) in zip(self.state.iter(), driver.iter()) {
                        *led = (*state_led).into()
                    }
                    driver.controller.render()?;
                },
                LightsCommand::Fill(color) => {
                    trace!("Setting all lights to color: (r:{}, g:{}, b:{})", color.r, color.g, color.b);
                    for state_led in self.state.iter_mut() {
                        *state_led = color;
                    }
                    driver.fill(color)?;
                },
                LightsCommand::SetSingle(index, color ) => {
                    trace!("Setting light number {} to color: (r:{}, g:{}, b:{})", index, color.r, color.g, color.b);
                    self.state[index] = color;
                    driver[index] = color.into();
                    driver.controller.render()?;
                },
                LightsCommand::Set(colors) => {
                    trace!("Setting lights to received colors");
                    for (state_led, (color, led)) in zip(self.state.iter_mut(), zip(colors, driver.iter())) {
                        *state_led = color;
                        *led = color.into();
                    }
                    driver.controller.render()?;
                },
                LightsCommand::ChangeConfig(config) => {
                    trace!("Making new config");
                    driver.clear()?;
                    driver = LedDriver::new(config)?;
                    for (state_led, led) in zip(self.state.iter(), driver.iter()) {
                        *led = (*state_led).into()
                    }
                    driver.controller.render()?;
                    self.config = config;
                }
                LightsCommand::Stop => {
                    debug!("Stopping lights controller");
                    driver.clear()?;
                    break
                },
            }
        }
        Ok(())
    }
}

pub struct FakeLightsController {
    config: DriverConfig,
    receiver: mpsc::Receiver<LightsCommand>,
    state: Vec<LedColor>,
}

impl LightsController for FakeLightsController {
    fn new(config: DriverConfig, receiver: mpsc::Receiver<LightsCommand>) -> impl LightsController {
        let mut default_colors: Vec<LedColor> = Vec::with_capacity(config.left + config.right);
        let red: LedColor = [128, 0, 0].into();
        let green: LedColor = [0, 128, 0].into();
        let white: LedColor = [96, 96, 64].into();
        for i in 0..(config.left + config.right) {
            if i%3 == 0 {
                default_colors.push(red);
            } else if i%3 == 1 {
                default_colors.push(green);
            } else if i%3 == 2 {
                default_colors.push(white);
            }
        }
        FakeLightsController { config, receiver, state: default_colors }
    }

    /// Consume the controller and start a loop to control the lights based on MPSC messages
    async fn start(&mut self) -> Result<()> {
        debug!("Starting dev lights controller");
        while let Some(cmd) = self.receiver.recv().await {
            match cmd {
                LightsCommand::Off => {
                    debug!("Turning lights off");
                },
                LightsCommand::On => {
                    debug!("Turning lights on");
                },
                LightsCommand::Fill(color) => {
                    debug!("Setting all lights to color: (r:{}, g:{}, b:{})", color.r, color.g, color.b);
                    for state_led in self.state.iter_mut() {
                        *state_led = color;
                    }
                },
                LightsCommand::SetSingle(index, color ) => {
                    debug!("Setting light number {} to color: (r:{}, g:{}, b:{})", index, color.r, color.g, color.b);
                    self.state[index] = color;
                },
                LightsCommand::Set(colors) => {
                    debug!("Setting lights to received colors");
                    for (state_led, color) in zip(self.state.iter_mut(), colors) {
                        *state_led = color;
                    }
                },
                LightsCommand::ChangeConfig(config) => {
                    debug!("Making new config");
                    self.config = config;
                }
                LightsCommand::Stop => {
                    debug!("Stopping lights controller");
                    break
                },
            }
        }
        Ok(())
    }
}