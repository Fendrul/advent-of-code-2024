pub struct AntennaCoordinates {
    coordinates: Vec<(isize, isize)>,
}

impl AntennaCoordinates {
    pub fn new() -> Self {
        Self {
            coordinates: Vec::new(),
        }
    }

    pub fn add_coordinates(&mut self, coordinates: (isize, isize)) {
        self.coordinates.push(coordinates);
    }

    pub fn get_extrapolated_coordinates(&self) -> Vec<(isize, isize)> {
        let mut extrapolated_coordinates = Vec::new();

        calculate_coordinates(&mut extrapolated_coordinates, &self.coordinates);

        extrapolated_coordinates
    }

    pub fn get_extrapolated_coordinates_with_repetition(
        &self,
        max_x: isize,
        max_y: isize,
    ) -> Vec<(isize, isize)> {
        let mut extrapolated_coordinates = Vec::new();

        calculate_coordinates_with_repetition(
            &mut extrapolated_coordinates,
            &self.coordinates,
            max_x,
            max_y,
        );

        extrapolated_coordinates
            .into_iter()
            .filter(|(x, y)| (0..max_x).contains(x) && (0..max_y).contains(y))
            .collect()
    }
}

fn calculate_coordinates_with_repetition(
    extrapolated_coordinates: &mut Vec<(isize, isize)>,
    antenna_coordinates: &[(isize, isize)],
    max_x: isize,
    max_y: isize,
) {
    if antenna_coordinates.len() > 1 {
        let current_coordinates = antenna_coordinates[0];

        for coordinate in antenna_coordinates[1..].iter() {
            let x_diff = coordinate.0 - current_coordinates.0;
            let y_diff = coordinate.1 - current_coordinates.1;

            let mut x_extrapolated = coordinate.0;
            let mut y_extrapolated = coordinate.1;

            while (0..max_x).contains(&x_extrapolated) && (0..max_y).contains(&y_extrapolated) {
                extrapolated_coordinates.push((x_extrapolated, y_extrapolated));

                x_extrapolated += x_diff;
                y_extrapolated += y_diff;
            }

            let mut x_extrapolated = current_coordinates.0;
            let mut y_extrapolated = current_coordinates.1;

            while (0..max_x).contains(&x_extrapolated) && (0..max_y).contains(&y_extrapolated) {
                extrapolated_coordinates.push((x_extrapolated, y_extrapolated));

                x_extrapolated -= x_diff;
                y_extrapolated -= y_diff;
            }

            calculate_coordinates_with_repetition(
                extrapolated_coordinates,
                &antenna_coordinates[1..],
                max_x,
                max_y,
            );
        }
    }
}

fn calculate_coordinates(
    extrapolated_coordinates: &mut Vec<(isize, isize)>,
    antenna_coordinates: &[(isize, isize)],
) {
    if antenna_coordinates.len() > 1 {
        let current_coordinates = antenna_coordinates[0];

        for coordinate in antenna_coordinates[1..].iter() {
            let x_diff = coordinate.0 - current_coordinates.0;
            let y_diff = coordinate.1 - current_coordinates.1;

            let x_extrapolated = coordinate.0 + x_diff;
            let y_extrapolated = coordinate.1 + y_diff;

            extrapolated_coordinates.push((x_extrapolated, y_extrapolated));

            let x_extrapolated = current_coordinates.0 - x_diff;
            let y_extrapolated = current_coordinates.1 - y_diff;

            extrapolated_coordinates.push((x_extrapolated, y_extrapolated));

            calculate_coordinates(extrapolated_coordinates, &antenna_coordinates[1..]);
        }
    }
}
