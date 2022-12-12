use super::{instruction::Instruction, register::Register};

pub struct SpriteRenderer {
    sprite_position: Register,
    pixels: Vec<Vec<char>>,
}

impl SpriteRenderer {
    pub fn new(sprite_start_position: i64) -> Self {
        Self {
            sprite_position: Register::new(sprite_start_position),
            pixels: Vec::<Vec<char>>::new(),
        }
    }

    pub fn draw(&mut self, instructions: &mut Vec<Instruction>) -> Result<(), String> {
        for cycle in 1..=240 {
            self.sprite_position.apply_instructions(instructions, cycle);

            match self.pixels.iter().last() {
                Some(row) => {
                    if row.len() == 40 {
                        self.pixels.push(Vec::<char>::new());
                    }
                }
                None => {
                    self.pixels.push(Vec::<char>::new());
                }
            };

            let pixel_row = self
                .pixels
                .iter_mut()
                .last()
                .ok_or_else(|| "No pixel rows found in sprite renderer".to_string())?;

            let current_pixel_position = i64::try_from(pixel_row.len()).map_err(|_| {
                "Failed to parse current pixel position to 64-bit integer".to_string()
            })?;

            let current_sprite_position = self.sprite_position.value;

            if current_pixel_position == current_sprite_position
                || current_pixel_position == current_sprite_position - 1
                || current_pixel_position == current_sprite_position + 1
            {
                pixel_row.push('#');
            } else {
                pixel_row.push('.');
            }
        }

        Ok(())
    }
}

impl ToString for SpriteRenderer {
    fn to_string(&self) -> String {
        self.pixels
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}
