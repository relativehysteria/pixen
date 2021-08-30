use macroquad::prelude::*;

use ron::de::from_reader;
use serde::Deserialize;

use std::fs::File;


pub const CONFIG_NAME: &str = "config.ron";

/// Checks whether the debug key is held
pub fn debug_key_held() -> bool {
    is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)
}

#[derive(Deserialize)]
pub struct GameConfig {
    /// The number of pixels to play with.
    pub num_pixels: usize,

    /// Physics config
    pub phy: PhysicsConfig,

    /// Graphics config
    pub gfx: GraphicsConfig,

    /// Debug config
    pub dbg: DebugConfig,
}

impl GameConfig {
    /// Reads the file `filename`, parses the config and returns it
    pub fn read_config(filename: &str) -> Self {
        // Parse the config file
        let config = File::open(filename)
            .expect("Couldn't open the configuration file.");
        let config: GameConfig = from_reader(config)
            .expect("Couldn't parse the configuration file");

        // Some assertions so that the sandbox doesn't go crazy
        assert!(config.phy.max_velocity   >= 0.);
        assert!(config.phy.friction       >= 0.);
        assert!(config.phy.acceleration   >= 0.);
        assert!(config.phy.friction       <  config.phy.acceleration);
        assert!(config.gfx.min_brightness <= config.gfx.max_brightness);
        assert!(config.gfx.pixel_size     >  0.);

        config
    }
}

#[derive(Deserialize)]
pub struct PhysicsConfig {
    /// Maximum velocity of the pixels
    pub max_velocity: f32,

    /// Friction of the pixels
    pub friction: f32,

    /// Acceleration of the pixels towards/away from a gravity field, if they
    /// are affected by it
    pub acceleration: f32,

    /// Area of effect of gravity fields
    pub gravity_field_aoe: f32,
}

#[derive(Deserialize)]
pub struct GraphicsConfig {
    /// Size of a single pixel
    pub pixel_size: f32,

    /// Minimal brightness level of the pixels
    pub min_brightness: u8,

    /// Maximal brightness level of the pixels
    pub max_brightness: u8,
}

#[derive(Deserialize)]
pub struct DebugConfig {
    /// Whether to automatically show debug info on pause
    pub on_pause: bool,

    /// Whether to show FPS in debug info
    pub fps: bool,

    /// Whether to draw gravity fields
    pub draw_fields: bool,

    /// Whether to show the current number of fields in the arena
    pub n_fields: bool,
}
