pub struct Droplet {
    pub cubes: Vec<Vec<Vec<Cube>>>,
}

pub enum Cube {
    Lava,
    Air,
}

pub type Coordinates = [usize; 3];

impl Droplet {
    pub fn surface_area(&self, exterior_only: bool) -> usize {
        self.iter_coordinates()
            .filter_map(|coordinates| {
                if !matches!(self.get(coordinates), Some(Cube::Lava)) {
                    return None;
                }

                Some(self.exposed_sides_of_cube(&coordinates, exterior_only))
            })
            .sum()
    }

    fn get(&self, [x, y, z]: Coordinates) -> Option<&Cube> {
        self.cubes.get(x)?.get(y)?.get(z)
    }

    fn exposed_sides_of_cube(&self, coordinates: &Coordinates, exterior_only: bool) -> usize {
        let neighbors = self.neighbors_of_cube(coordinates);

        let exposed_to_outside = 6 - neighbors.len();

        let air_neighbor_count = neighbors
            .iter()
            .filter(|(cube, coordinates)| {
                if !matches!(cube, Cube::Air) {
                    return false;
                }

                if exterior_only
                    && !self.cube_exposed_to_outside(*coordinates, &mut Vec::<Coordinates>::new())
                {
                    return false;
                }

                true
            })
            .count();

        exposed_to_outside + air_neighbor_count
    }

    fn neighbors_of_cube(&self, coordinates: &Coordinates) -> Vec<(&Cube, Coordinates)> {
        let [x, y, z] = *coordinates;

        let mut neighbor_coordinates = vec![[x + 1, y, z], [x, y + 1, z], [x, y, z + 1]];

        if x != 0 {
            neighbor_coordinates.push([x - 1, y, z]);
        }
        if y != 0 {
            neighbor_coordinates.push([x, y - 1, z]);
        }
        if z != 0 {
            neighbor_coordinates.push([x, y, z - 1]);
        }

        neighbor_coordinates
            .iter()
            .flat_map(|&coordinates| {
                let cube = self.get(coordinates)?;
                Some((cube, coordinates))
            })
            .collect::<Vec<(&Cube, Coordinates)>>()
    }

    fn cube_exposed_to_outside(
        &self,
        coordinates: Coordinates,
        checked_coordinates: &mut Vec<Coordinates>,
    ) -> bool {
        let neighbors = self.neighbors_of_cube(&coordinates);

        if neighbors.len() < 6 {
            return true;
        }

        checked_coordinates.push(coordinates);

        let mut exposed_to_outside = false;

        for (cube, coordinates) in neighbors {
            if matches!(cube, Cube::Air)
                && !checked_coordinates.contains(&coordinates)
                && self.cube_exposed_to_outside(coordinates, checked_coordinates)
            {
                exposed_to_outside = true;
            }
        }

        exposed_to_outside
    }

    fn iter_coordinates(&self) -> impl Iterator<Item = Coordinates> + '_ {
        self.cubes.iter().enumerate().flat_map(|(x, y_vec)| {
            y_vec
                .iter()
                .enumerate()
                .flat_map(move |(y, z_vec)| z_vec.iter().enumerate().map(move |(z, _)| [x, y, z]))
        })
    }
}
