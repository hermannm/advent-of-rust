use super::sensor_map::{Position, Sensor, SensorMap};

impl TryFrom<&str> for SensorMap {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let sensors = input
            .lines()
            .map(Sensor::try_from)
            .collect::<Result<Vec<Sensor>, String>>()?;

        Ok(SensorMap { sensors })
    }
}

impl TryFrom<&str> for Sensor {
    type Error = String;

    fn try_from(input_line: &str) -> Result<Self, Self::Error> {
        let (_, rest_of_line) = input_line
            .split_once("Sensor at ")
            .ok_or_else(|| format!("Expected to find 'Sensor at' in input line '{input_line}'"))?;

        let (sensor_position_string, beacon_position_string) = rest_of_line
            .split_once(": closest beacon is at ")
            .ok_or_else(|| {
                format!("Expected to find 'closest beacon is at' in input line '{input_line}'")
            })?;

        let sensor_position = Position::try_from(sensor_position_string)?;
        let beacon_position = Position::try_from(beacon_position_string)?;
        let detection_distance = sensor_position.distance_to(&beacon_position);

        Ok(Sensor {
            position: sensor_position,
            beacon_position,
            detection_distance,
        })
    }
}

impl TryFrom<&str> for Position {
    type Error = String;

    fn try_from(position_string: &str) -> Result<Self, Self::Error> {
        let (x_string, y_string) = position_string.split_once(", ").ok_or_else(|| {
            format!("Expected to find comma separator in position string: {position_string}")
        })?;

        let parse_integer_with_prefix = |input_string: &str, prefix: &str| -> Result<i32, String> {
            let (_, integer_string) = input_string.split_once(prefix).ok_or_else(|| {
                format!("Failed to find prefix '{prefix}' in input string '{input_string}'")
            })?;

            integer_string
                .parse::<i32>()
                .map_err(|_| format!("Failed to parse '{integer_string}' to integer"))
        };

        let x = parse_integer_with_prefix(x_string, "x=")?;
        let y = parse_integer_with_prefix(y_string, "y=")?;

        Ok(Position { x, y })
    }
}
