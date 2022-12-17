pub struct Sensor {
    pub position: Position,
    pub beacon_position: Position,
    pub detection_distance: i32,
}

pub struct SensorMap {
    pub sensors: Vec<Sensor>,
}

impl SensorMap {
    pub fn find_first_possible_beacon(
        &self,
        x_range: (i32, i32),
        y_range: (i32, i32),
    ) -> Option<Position> {
        for y in y_range.0..y_range.1 {
            let mut x = x_range.0;

            'x_loop: while x <= x_range.1 {
                let position = Position { x, y };

                for sensor in &self.sensors {
                    let distance_to_sensor = position.distance_to(&sensor.position);

                    if distance_to_sensor <= sensor.detection_distance {
                        let distance_to_detection_limit =
                            sensor.detection_distance - distance_to_sensor;

                        if position.x < sensor.position.x {
                            x += (sensor.position.x - position.x) * 2
                                + distance_to_detection_limit
                                + 1;
                        } else {
                            x += distance_to_detection_limit + 1;
                        }

                        continue 'x_loop;
                    }
                }

                return Some(position);
            }
        }

        None
    }

    pub fn count_positions_without_beacons(
        &self,
        positions_to_check: impl Iterator<Item = Position>,
    ) -> usize {
        positions_to_check
            .filter(|position| {
                self.sensors.iter().any(|sensor| {
                    sensor.beacon_position != *position
                        && position.distance_to(&sensor.position) <= sensor.detection_distance
                })
            })
            .count()
    }

    pub fn get_positions_in_row(
        &self,
        row_y_value: i32,
    ) -> Result<impl Iterator<Item = Position>, String> {
        let x_iterator = self
            .sensors
            .iter()
            .flat_map(|sensor| [sensor.position.x, sensor.beacon_position.x].into_iter());

        let min_x = x_iterator
            .clone()
            .min()
            .ok_or_else(|| String::from("Failed to find minimum x value in sensor map"))?;

        let max_x = x_iterator
            .max()
            .ok_or_else(|| String::from("Failed to find maximum x value in sensor map"))?;

        let halfway_distance = ((min_x.abs() + max_x.abs()) / 2) + 1;

        let positions_in_row = ((min_x - halfway_distance)..=(max_x + halfway_distance))
            .into_iter()
            .map(move |x| Position { x, y: row_y_value });

        Ok(positions_in_row)
    }
}

#[derive(PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn tuning_signal(&self) -> i64 {
        i64::from(self.x) * 4_000_000i64 + i64::from(self.y)
    }
}
