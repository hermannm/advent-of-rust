use super::droplet::{Coordinates, Cube, Droplet};

impl TryFrom<&str> for Droplet {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let lava_coordinates = input
            .lines()
            .map(coordinates_from_line)
            .collect::<Result<Vec<Coordinates>, String>>()?;

        let mut highest_x: usize = 0;
        let mut highest_y: usize = 0;
        let mut highest_z: usize = 0;

        for [x, y, z] in &lava_coordinates {
            if *x > highest_x {
                highest_x = *x;
            }
            if *y > highest_y {
                highest_y = *y;
            }
            if *z > highest_z {
                highest_z = *z;
            }
        }

        let mut cubes = Vec::<Vec<Vec<Cube>>>::new();

        for _ in 0..=highest_x {
            let mut y_vec = Vec::<Vec<Cube>>::new();

            for _ in 0..=highest_y {
                let mut z_vec = Vec::<Cube>::new();

                for _ in 0..=highest_z {
                    z_vec.push(Cube::Air);
                }

                y_vec.push(z_vec);
            }

            cubes.push(y_vec);
        }

        for [x, y, z] in &lava_coordinates {
            cubes[*x][*y][*z] = Cube::Lava;
        }

        Ok(Droplet { cubes })
    }
}

fn coordinates_from_line(input_line: &str) -> Result<Coordinates, String> {
    let numbers = input_line
        .split(',')
        .map(|number_string| {
            number_string
                .parse::<usize>()
                .map_err(|_| format!("Failed to parse '{number_string}' to integer"))
        })
        .collect::<Result<Vec<usize>, String>>()?;

    Coordinates::try_from(numbers)
        .map_err(|_| format!("Input line '{input_line}' did not contain 3 integers"))
}
