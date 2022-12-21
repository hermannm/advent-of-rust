use std::collections::{hash_map::Iter, HashMap};

pub const NUMBER_OF_ROBOTS: usize = 4;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Robot {
    OreCollector,
    ClayCollector,
    ObsidianCollector,
    GeodeCracker,
}

#[derive(Clone)]
pub struct Robots {
    map: HashMap<Robot, u16>,
}

impl Robots {
    pub fn get(&self, robot: &Robot) -> u16 {
        *self.map.get(robot).unwrap_or(&0)
    }

    pub fn add(&mut self, robot: Robot) {
        let robot_count = self.map.entry(robot).or_insert(0);
        *robot_count += 1;
    }
}

impl Default for Robots {
    fn default() -> Self {
        let mut map = HashMap::<Robot, u16>::with_capacity(NUMBER_OF_ROBOTS);
        map.insert(Robot::OreCollector, 1);
        Self { map }
    }
}

impl<'a> IntoIterator for &'a Robots {
    type Item = (&'a Robot, &'a u16);

    type IntoIter = Iter<'a, Robot, u16>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}
