use macroquad::prelude::*;
use ron::de::from_reader;
use serde::Deserialize;

use pixen::vector::*;
use pixen::rng::*;
use pixen::pixel::*;
use pixen::gravity_field::*;

use std::fs::File;


const CONFIG_NAME: &str = "config.ron";

#[derive(Deserialize)]
struct GameConfig {
    /// The number of pixels to play with.
    num_pixels: usize,

    /// Minimal brightness level of the pixels
    min_brightness: u8,

    /// Maximal brightness level of the pixels
    max_brightness: u8,

    /// Maximum velocity of the pixels
    max_velocity: f32,

    /// Acceleration of the pixels towards/away from a gravity field, if they
    /// are affected by it
    acceleration: f32,

    /// Area of effect of gravity fields
    gravity_field_aoe: f32,

    /// Gravity fields will only be drawn if this is true
    draw_fields_only_when_paused: bool,

    /// Friction of the pixels
    friction: f32,
}

impl GameConfig {
    /// Reads the file `filename`, parses the config and returns it
    fn read_config(filename: &str) -> Self {
        // Parse the config file
        let config = File::open(filename)
            .expect("Couldn't open the configuration file.");
        let config: GameConfig = from_reader(config)
            .expect("Couldn't parse the configuration file");

        // Some assertions so that the sandbox doesn't go crazy
        assert!(config.max_velocity   >= 0.);
        assert!(config.friction       >= 0.);
        assert!(config.acceleration   >= 0.);
        assert!(config.friction       <= config.acceleration);
        assert!(config.min_brightness <= config.max_brightness);

        config
    }
}

/// The game field which is used for the game
struct GameField {
    /// Pixels in the arena
    pixels: Vec<Pixel>,

    /// Gravity fields in the arena
    gravity_fields: Vec<GravityField>,

    /// The game RNG
    rng: Rng,

    /// Whether the game is paused
    is_paused: bool,

    /// The game configuration
    config: GameConfig,
}

impl GameField {
    /// Creates a new GameField, spawning the amount of pixels defined by config
    /// and populating the `pixels` vec with them.
    fn new(config: GameConfig) -> Self {
        // Create the struct fields
        let mut temp = Self {
            pixels:         vec![],
            gravity_fields: vec![],
            rng:            Rng::new(),
            is_paused:      false,
            config,
        };
        temp.populate_pixels();
        temp
    }

    /// Populates the inner `pixels` vector with the amount of pixels defined
    /// by config.
    fn populate_pixels(&mut self) {
        self.pixels = Vec::with_capacity(self.config.num_pixels);

        // Spawn the pixels and put them inside the `pixels` vec
        for _ in 0..self.config.num_pixels {
            let pos_x = (self.rng.rand() % screen_width()  as u64) as f32;
            let pos_y = (self.rng.rand() % screen_height() as u64) as f32;
            self.pixels.push(Pixel::new(Vector::new(pos_x, pos_y)));
        }
    }

    /// Updates the game state and ticks the physics engine once.
    ///
    /// * Escape resets the arena
    /// * Space pauses the arena (new gravity fields can still be spawned).
    /// * LMB press creates an attracting gravity field,
    /// * RMB press creates a repelling gravity field.
    fn update(&mut self) {
        let mouse_pos = Vector::coords(mouse_position());

        // LMB press creates an attracting gravity field,
        // RMB press creates a repelling gravity field.
        if is_mouse_button_pressed(MouseButton::Left) {
            self.gravity_fields.push(
                GravityField::new(
                    mouse_pos,
                    self.config.gravity_field_aoe,
                    self.config.acceleration,
                )
            );
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.gravity_fields.push(
                GravityField::new(
                    mouse_pos,
                    self.config.gravity_field_aoe,
                    -self.config.acceleration,
                )
            );
        }

        // Escape resets the arena
        if is_key_pressed(KeyCode::Escape) {
            self.config = GameConfig::read_config(CONFIG_NAME);
            self.populate_pixels();
            self.gravity_fields.clear();
        }

        // Space pauses the arena
        if is_key_pressed(KeyCode::Space) {
            self.is_paused = !self.is_paused;
        }
        if self.is_paused {
            return;
        }

        // Update the state of pixels
        let mut accelerations: Vec<Vector> = vec![];
        for px in self.pixels.iter_mut() {
            // Calculate the direction and acceleration of this pixel
            accelerations.clear();
            for field in &self.gravity_fields {
                if !field.in_aoe(&px.position) {
                    continue;
                }

                let mut direction = field.position - px.position;
                direction.normalize();
                accelerations.push(direction * Vector::from(field.strength));
            }
            let acceleration = accelerations.iter().fold(
                Vector::from(0.), |acc, x| acc + *x
            );

            // Create friction - inverted and normalized velocity.
            // We normalize it so that it is easy to scale.
            let mut friction = px.velocity;
            friction.normalize();
            friction *= Vector::from(-1.);
            friction *= Vector::from(self.config.friction);

            // Apply the forces to velocity
            px.velocity += acceleration;
            px.velocity += friction;

            // Limit the velocity and apply it
            px.velocity.limit(self.config.max_velocity);
            px.position += px.velocity;
        }
    }

    #[allow(dead_code)]
    /// Keeps the pixels on the screen (within its bounds, that is width/height)
    fn keep_within_bounds(&mut self) {
        for px in self.pixels.iter_mut() {
            if px.position.x > screen_width() || px.position.x < 0. {
                px.velocity.x *= -1.;
            }
            if px.position.y > screen_height() || px.position.y < 0. {
                px.velocity.y *= -1.;
            }

            px.position.x = px.position.x.clamp(0., screen_width());
            px.position.y = px.position.y.clamp(0., screen_height());
        }
    }

    #[allow(dead_code)]
    /// When pixels reach one edge, their location is set to the other.
    /// This behavior is equal to the one in Snake (when the arena is unbounded)
    fn snake_bounds(&mut self) {
        for px in self.pixels.iter_mut() {
            if px.position.x > screen_width() {
                px.position.x = 0.;
            } else if px.position.x < 0. {
                px.position.x = screen_width();
            }

            if px.position.y > screen_height() {
                px.position.y = 0.;
            } else if px.position.y < 0. {
                px.position.y = screen_height();
            }
        }
    }

    /// Renders the pixels on the screen
    fn render(&mut self) {
        clear_background(BLACK);

        if self.is_paused {
            draw_text("PAUSED", 0., 20., 32., WHITE);
        }

        // Draw gravity fields.
        // Attractive fields are green, repelling fields are red.
        let draw_when_paused = self.config.draw_fields_only_when_paused;
        if (draw_when_paused && self.is_paused) || (!draw_when_paused) {
            for field in self.gravity_fields.iter() {
                let color = if field.strength.is_sign_negative() {
                    RED
                } else {
                    GREEN
                };

                draw_circle(field.position.x, field.position.y, 10., color);
            }
        }

        // Draw pixels
        for px in self.pixels.iter() {
            // Pixels have a random brightness every frame
            let px_color = self.rng.range(
                self.config.min_brightness as u64,
                self.config.max_brightness as u64
            ) as u8;
            let px_color = Color::from_rgba(px_color, px_color, px_color, 255);

            draw_circle(px.position.x, px.position.y, 0.75, px_color);
        }
    }

}

#[macroquad::main("Pixen")]
async fn main() {
    // Parse and create the config
    let config = GameConfig::read_config(CONFIG_NAME);

    // Create the game_field and start the game
    let mut game_field = GameField::new(config);

    '_gameloop: loop {
        game_field.update();
        //game_field.keep_within_bounds();
        game_field.snake_bounds();
        game_field.render();
        next_frame().await
    }
}
