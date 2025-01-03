use std::{iter::zip, sync::Arc};
use tokio::sync::{mpsc, watch, RwLock};
use anyhow::{bail, Context, Result};
#[allow(unused_imports)]
use log::{trace, debug, info, warn, error};

use crate::appstate::{AppState, AppStateChange};

mod color;
pub use color::LedColor;
pub mod modes;
use modes::{DriverCommand, LightsMode, MODES_PARAMS_DIR};
pub mod params;
pub use params::Param;
mod driver;
use driver::{DriverConfig, LedDriver};

pub struct LightsModesList {
    pub names: Vec<String>,
    modes: Vec<Arc<RwLock<dyn LightsMode>>>,
    selected: usize,
}

impl LightsModesList {
    pub fn new() -> Self {
        LightsModesList { modes: vec![], names: vec![], selected: 0 }
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Register a new mode with the lights-controller object
    pub fn register<M: 'static + LightsMode>(&mut self, new_mode: M) {
        self.names.push(new_mode.get_name());
        self.modes.push(Arc::new( RwLock::new(new_mode)));
    }
    
    /// Set the active mode and return a reference to the mode
    pub fn select(&mut self, index: usize) -> Option<Arc<RwLock<dyn LightsMode>>> {
        self.selected = index;
        self.modes.get(index).map(|x| x.clone())
    }
}

pub struct LightsController {
    driver: LedDriver,
    state_tx: watch::Sender<AppState>,
    cmd_rx: mpsc::Receiver<AppStateChange>,
    on_off: bool,
    // modes: Vec<Arc<RwLock<dyn LightsMode>>>,
    // selected: usize,
}

impl LightsController {
    pub fn new(
        state_tx: watch::Sender<AppState>, 
        cmd_rx: mpsc::Receiver<AppStateChange>,
    ) -> Result<(Self, LightsModesList)> {
        let driver_config = DriverConfig::init();
        Ok((
            LightsController {
                driver: LedDriver::new(driver_config)?, 
                state_tx, 
                cmd_rx,
                on_off: true,
            },
            LightsModesList::new(),
        ))
    }

    /// Pass a driver command onto the lights ColorDriver
    fn process_driver_cmd(&mut self, drv_cmd: &DriverCommand) {
        match drv_cmd {
            DriverCommand::Clear => {
                if let Err(e) = self.driver.clear() {
                    warn!("LED Driver Error on clear: {e:?}");
                }
            },
            DriverCommand::Fill(color) => {
                if let Err(e ) = self.driver.fill(*color) {
                    warn!("LED Driver Error on fill: {e:?}");
                }
            },
            DriverCommand::SetAll(colors) => {
                for (led, color) in zip(self.driver.iter(), colors) {
                    *led = (*color).into();
                }
                self.driver.render();
            },
            DriverCommand::SetSome(pairs) => {
                let num_leds = self.driver.size();
                for (index, color) in pairs {
                    if *index < num_leds {
                        self.driver[*index] = (*color).into();
                    }
                }
                self.driver.render();
            },
        }        
    }

    /// Transmit the current app state over the watch channel
    fn update_state(&self, modes: Vec<String>, selected: usize, params: Vec<Param>) {
        let app_state = AppState::new(self.on_off, modes, selected,  params);
        let _ = self.state_tx.send_replace(app_state);
    }

    /// Run the main loop of the back-end lights controller
    pub async fn run(&mut self, mut modes_list: LightsModesList) -> Result<()> {
        // make sure we have modes to run!
        if modes_list.len() == 0 {
            bail!("Need at least one mode to run!");
        }
        // Get a list of mode strings
        let mode_names = modes_list.names.clone();
        // read the active mode from file or default to zero
        let active_mode_file = std::path::PathBuf::from(MODES_PARAMS_DIR).join("active.txt");
        let index = match tokio::fs::read_to_string(&active_mode_file).await {
            Ok(index_str) => {
                match usize::from_str_radix(&index_str, 10) {
                    Ok(index) => {
                        if index < modes_list.len() {
                            index
                        } else {
                            warn!("Error setting active mode, saved move ({}) is larger than mode list ({})", index, modes_list.len());
                            0
                        }
                    },
                    Err(e) => {
                        warn!("Error reading index from active mode file: {e:?}");
                        0
                    }
                }
            },
            Err(e) => {
                warn!("Error reading active mode file: {e:?}");
                0
            }
        };
        // set the active mode and initialize
        let mut selected = index;
        let mut active_mode_arc = modes_list.select(index).context("Could not select active mode")?;
        let mut active_mode = active_mode_arc.write().await;
        let drv_cmd = active_mode.init().await;
        // proccess the initial driver command
        self.process_driver_cmd(&drv_cmd);
        // get the parameters and update the state
        let params = active_mode.get_params();
        self.update_state(mode_names.clone(), selected, params);
        // start our main loop
        loop {
            tokio::select! {
                // Get stat-change command over the MPSC from the web-sockets
                state_change_cmd = self.cmd_rx.recv() => {
                    // process the state-change command into a driver command
                    let drv_cmd = match state_change_cmd {
                        None => {
                            // channel closed!
                            bail!("Command MPSC channel closed!");
                        },
                        Some(AppStateChange::OnOff(change)) => {
                            self.on_off = change;
                            active_mode.on_off(change).await
                        },
                        Some(AppStateChange::ModeSelect(new_index)) => {
                            // stop the current mode
                            active_mode.stop().await;
                            // set new active mode
                            if new_index < modes_list.len() {
                                selected = new_index;
                                drop(active_mode);
                                active_mode_arc = modes_list.select(index).context("Could not select active mode")?;
                                active_mode = active_mode_arc.write().await;                        
                                if let Err(e) = tokio::fs::write(&active_mode_file, format!("{}", selected)).await {
                                    warn!("Error saving new active mode: {e:?}");
                                }
                            }
                            // initialize new mode
                            active_mode.init().await
                        },
                        Some(AppStateChange::ChangeParam(param)) => {
                            let (opt_drv_conf, drv_cmd) = active_mode.update_param(param).await;
                            if let Some(drv_conf) = opt_drv_conf {
                                // only change driver if we're really controlling LEDs
                                active_mode.stop().await;
                                // drop(self.driver);
                                match LedDriver::new(drv_conf) {
                                    Ok(new_drv) => { self.driver = new_drv; },
                                    Err(e) => {
                                        bail!("Failed resetting LED driver {e:?}");
                                    }
                                }
                                let drv_cmd = active_mode.init().await;
                                self.process_driver_cmd(&drv_cmd);                                    
                            }
                            drv_cmd
                        },
                        Some(AppStateChange::Stop) => {
                            active_mode.stop().await;
                            break;
                        }
                    };
                    // process the command
                    self.process_driver_cmd(&drv_cmd);
                    // get the (possibly changed) set of parameters
                    let params = active_mode.get_params();
                    self.update_state(mode_names.clone(), selected, params);
                },
                drv_cmd = active_mode.tick() => {
                    self.process_driver_cmd(&drv_cmd);
                    // get the (possibly changed) set of parameters
                    let params = active_mode.get_params();
                    self.update_state(mode_names.clone(), selected, params);
                }
            }
        }
        Ok(())
    }
}