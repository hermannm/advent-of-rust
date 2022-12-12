use super::{instruction::Instruction, sprite_renderer::SpriteRenderer};

pub fn solve_puzzle(input: &str) -> Result<String, String> {
    let mut instructions = input
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>, String>>()?;

    let mut sprite_renderer = SpriteRenderer::new(1);

    sprite_renderer.draw(&mut instructions)?;

    Ok(format!("\n{}", sprite_renderer.to_string()))
}
