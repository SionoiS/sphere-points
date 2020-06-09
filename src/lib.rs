const HALFPI: f64 = std::f64::consts::PI / 2.0;

/// Calculate the number of coordinates.
pub fn point_count_1_d(radius_range: u16, radius_resolution: u16) -> u16 {
    1 + radius_range / radius_resolution
}

/// Calculate the number of coordinates.
pub fn point_count_2_d(
    radius_range: u16,
    radius_resolution: u16,
    longitude_range: u16,
    longitude_resolution: u16,
) -> u16 {
    let radius_steps = radius_range / radius_resolution;

    let mut longitude_steps = longitude_range / longitude_resolution + 1;

    if longitude_steps % 2 == 0 {
        //make even odd for symetry
        longitude_steps += 1;
    }

    if longitude_range == 360 {
        // remove duplicate meridian
        longitude_steps -= 1;
    }

    1 + radius_steps * longitude_steps
}

/// Calculate the number of coordinates.
pub fn point_count_3_d(
    radius_range: u16,
    radius_resolution: u16,
    longitude_range: u16,
    longitude_resolution: u16,
    latitude_range: u16,
    latitude_resolution: u16,
) -> u16 {
    let radius_steps = radius_range / radius_resolution;

    let mut longitude_steps = longitude_range / longitude_resolution + 1;

    if longitude_steps % 2 == 0 {
        //make even odd for symetry
        longitude_steps += 1;
    }

    if longitude_range == 360 {
        // remove duplicate meridian
        longitude_steps -= 1;
    }

    let mut latitude_steps = latitude_range / latitude_resolution + 1;

    if latitude_steps % 2 == 0 {
        //make even odd for symetry
        latitude_steps += 1;
    }

    if latitude_range == 180 {
        // remove duplicates
        latitude_steps -= 2;
    }

    1 + radius_steps * longitude_steps * latitude_steps
}

/// Calculate the cartesian coordinates of the points on a line. (1D)
pub fn calculate_1_d(radius_range: u16, radius_resolution: u16, slice: &mut [Coordinates]) {
    let radius_steps = radius_range / radius_resolution;

    let mut index = 1;
    for i in 1..radius_steps + 1 {
        let radius = radius_resolution * i;

        slice[index] = Coordinates {
            x: 0.0,
            y: 0.0,
            z: radius as f64,
        };
        index += 1;
    }
}

/// Calculate the cartesian coordinates of the points on a plane. (2D)
pub fn calculate_2_d(
    radius_range: u16,
    radius_resolution: u16,
    longitude_range: u16,
    longitude_resolution: u16,
    slice: &mut [Coordinates],
) {
    let radius_steps = radius_range / radius_resolution;
    let longitude_steps = longitude_range / longitude_resolution + 1;

    let mut longitude_stop = longitude_steps / 2;
    let longitude_start = -(longitude_stop as i16);

    let theta_resolution = (longitude_resolution as f64).to_radians();

    if longitude_range == 360 {
        // remove duplicate meridian
        longitude_stop -= 1;
    }

    let mut index = 1;
    for i in 1..radius_steps + 1 {
        let radius = (radius_resolution * i) as f64;

        for j in longitude_start..(longitude_stop + 1) as i16 {
            let theta = HALFPI + theta_resolution * j as f64;

            slice[index] = spherical_to_cartesian(radius, theta, HALFPI);
            index += 1;
        }
    }
}

/// Calculate the cartesian coordinates of the points in a sphere. (3D)
pub fn calculate_3_d(
    radius_range: u16,
    radius_resolution: u16,
    longitude_range: u16,
    longitude_resolution: u16,
    latitude_range: u16,
    latitude_resolution: u16,
    slice: &mut [Coordinates],
) {
    let radius_steps = radius_range / radius_resolution;
    let longitude_steps = longitude_range / longitude_resolution + 1;
    let latitude_steps = latitude_range / latitude_resolution + 1;

    let mut longitude_stop = longitude_steps / 2;
    let mut latitude_stop = latitude_steps / 2;
    let longitude_start = -(longitude_stop as i16);
    let mut latitude_start = -(latitude_stop as i16);

    let theta_resolution = (longitude_resolution as f64).to_radians();
    let phi_resolution = (latitude_resolution as f64).to_radians();

    if longitude_range == 360 {
        // remove duplicate meridian
        longitude_stop -= 1;
    }

    if latitude_range == 180 {
        // remove poles
        latitude_start += 1;
        latitude_stop -= 1;
    }

    //And finally radius
    let mut index = 1;
    for i in 1..radius_steps + 1 {
        let radius = (radius_resolution * i) as f64;

        //Then latitude
        for j in latitude_start..(latitude_stop + 1) as i16 {
            //0<=phi<=PI
            let phi = HALFPI + phi_resolution * j as f64;

            //Do longitude first
            for k in longitude_start..(longitude_stop + 1) as i16 {
                //0<=theta<=2PI
                let theta = HALFPI + theta_resolution * k as f64;

                slice[index] = spherical_to_cartesian(radius, theta, phi);
                index += 1;
            }
        }
    }
}

/// Axis Y and Z switched from math notation to something useable in game.
/// Normally in games Z is forward and Y is up.
fn spherical_to_cartesian(radius: f64, theta: f64, phi: f64) -> Coordinates {
    let sin_phi = phi.sin();

    //let x = radius * sin_phi * theta.cos();
    //let y = radius * sin_phi * theta.sin();
    //let z = radius * phi.cos();

    Coordinates {
        x: radius * sin_phi * theta.cos(),
        y: radius * phi.cos(),
        z: radius * sin_phi * theta.sin(),
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(test)]
mod tests {
    use crate::Coordinates;

    #[test]
    fn test_1_d() {
        use crate::calculate_1_d;
        use crate::point_count_1_d;

        let radius_range = 1000;
        let radius_resolution = 100;

        let count = point_count_1_d(radius_range, radius_resolution);

        let mut vector = vec![
            Coordinates {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            count as usize
        ];

        calculate_1_d(radius_range, radius_resolution, &mut vector);

        for point in vector.iter() {
            println!("{:#?}", point);
        }
    }

    #[test]
    fn test_2_d() {
        use crate::calculate_2_d;
        use crate::point_count_2_d;

        let radius_range = 1000;
        let radius_resolution = 100;
        let longitude_range = 180;
        let longitude_resolution = 30;

        let count = point_count_2_d(
            radius_range,
            radius_resolution,
            longitude_range,
            longitude_resolution,
        );

        let mut vector = vec![
            Coordinates {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            count as usize
        ];

        calculate_2_d(
            radius_range,
            radius_resolution,
            longitude_range,
            longitude_resolution,
            &mut vector,
        );

        for point in vector.iter() {
            println!("{:#?}", point);
        }
    }

    #[test]
    fn test_3_d() {
        use crate::calculate_3_d;
        use crate::point_count_3_d;

        let radius_range = 1000;
        let radius_resolution = 100;
        let longitude_range = 360;
        let longitude_resolution = 90;
        let latitude_range = 180;
        let latitude_resolution = 45;

        let count = point_count_3_d(
            radius_range,
            radius_resolution,
            longitude_range,
            longitude_resolution,
            latitude_range,
            latitude_resolution,
        );

        let mut vector = vec![
            Coordinates {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            count as usize
        ];

        calculate_3_d(
            radius_range,
            radius_resolution,
            longitude_range,
            longitude_resolution,
            latitude_range,
            latitude_resolution,
            &mut vector,
        );

        for point in vector.iter() {
            println!("{:#?}", point);
        }
    }
}
