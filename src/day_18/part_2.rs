use super::droplet::Droplet;

pub fn solve_puzzle(input: &str) -> Result<usize, String> {
    let droplet = Droplet::try_from(input)?;

    Ok(droplet.surface_area(true))
}
