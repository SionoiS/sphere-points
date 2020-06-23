use nalgebra::Vector3;
use std::f64::consts::{FRAC_PI_2, PI};

const TWO_PI: f64 = std::f64::consts::PI * 2.0;

/// Calculate the number of coordinates.
pub fn coordinates_count(
    radius_resolution: i32,
    longitude_range: f64,
    mut longitude_resolution: i32,
    latitude_range: f64,
    mut latitude_resolution: i32,
) -> i32 {
    if radius_resolution == 0 {
        return 1;
    }

    if longitude_resolution > 1 && longitude_range != 0.0 {
        let range = longitude_range.min(TWO_PI);
        let range = range.max(0.0);

        longitude_resolution += 1;

        if (range - TWO_PI).abs() < 0.0001 {
            // remove duplicate meridian
            longitude_resolution -= 1;
        }
    } else {
        longitude_resolution = 1;
    }

    if latitude_resolution > 1 && latitude_range != 0.0 {
        let range = latitude_range.min(PI);
        let range = range.max(0.0);

        latitude_resolution += 1;

        if (range - PI).abs() < 0.0001 {
            // remove duplicate poles
            latitude_resolution -= 2;
        }

        if latitude_resolution < 0 {
            latitude_resolution = 1;
        }
    } else {
        latitude_resolution = 1;
    }

    1 + radius_resolution * longitude_resolution * latitude_resolution
}

pub fn calculate_coordinates(
    radius_range: f64,
    radius_resolution: i32,
    longitude_range: f64,
    longitude_resolution: i32,
    latitude_range: f64,
    latitude_resolution: i32,
) -> Vec<Vector3<f64>> {
    //TODO process radii_vec, theta_vec, phi_vec in parallel???
    let radii_vec = {
        let step = radius_range / radius_resolution as f64;

        let count = radius_resolution as usize;

        let mut steps = Vec::with_capacity(count);

        for i in 1..radius_resolution + 1 {
            steps.push(step * i as f64);
        }

        //println!("radius_steps: {:#?}", steps);

        steps
    };

    let theta_vec = if longitude_resolution > 1 && longitude_range != 0.0 {
        let range = longitude_range.min(TWO_PI);
        let range = range.max(0.0);

        // skip duplicate meridian if full range
        let skip_last_meridian = (range - TWO_PI).abs() < 0.0001;

        let step = range / longitude_resolution as f64;
        let offset = range / 2.0;

        //println!("step: {}, offset: {}", step, offset);

        // count of separators between X equal parts.
        let count = longitude_resolution as usize + 1;

        let mut steps = Vec::with_capacity(count);

        for i in 0..count {
            if skip_last_meridian && i == count - 1 {
                continue;
            }

            steps.push(step * i as f64 - offset);
        }

        //println!("longitude_steps: {:#?}", steps);

        steps
    } else {
        vec![0.0]
    };

    let phi_vec = if latitude_resolution > 1 && latitude_range != 0.0 {
        let range = latitude_range.min(PI);
        let range = range.max(0.0);

        // skip north & south poles if full range
        let skip_poles = (range - PI).abs() < 0.0001;

        let step = range / latitude_resolution as f64;
        let offset = range / 2.0;

        //println!("step: {}, offset: {}", step, offset);

        // count of separators between X equal parts.
        let count = latitude_resolution as usize + 1;

        let mut steps = Vec::with_capacity(count);

        for i in 0..count {
            if skip_poles && (i == 0 || i == count - 1) {
                continue;
            }

            steps.push(step * i as f64 - offset);
        }

        //println!("latitude_steps: {:#?}", steps);

        steps
    } else {
        vec![0.0]
    };

    let mut vec = Vec::with_capacity(1 + radii_vec.len() * phi_vec.len() * theta_vec.len());

    vec.push(Vector3::new(0.0, 0.0, 0.0));

    //TODO process every coords in parallel???

    for radius in &radii_vec {
        for phi in &phi_vec {
            for theta in &theta_vec {
                //println!("Radius: {}, Phi: {}, Theta: {}", *radius, *phi, *theta);

                vec.push(spherical_to_cartesian(
                    *radius,
                    *theta + FRAC_PI_2,
                    *phi + FRAC_PI_2,
                ));
            }
        }
    }

    vec
}

fn spherical_to_cartesian(radius: f64, theta: f64, phi: f64) -> Vector3<f64> {
    let (sin_phi, cos_phi) = phi.sin_cos();
    let (sin_theta, cos_theta) = theta.sin_cos();

    //let x = radius * sin_phi * cos_theta;
    //let y = radius * sin_phi * sin_theta;
    //let z = radius * cos_phi;

    // Axis Y and Z switched from math notation to something useable in game.
    // Normally in games Z is forward and Y is up.
    Vector3::new(
        radius * sin_phi * cos_theta,
        radius * cos_phi,
        radius * sin_phi * sin_theta,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256StarStar;
    use std::f64::consts::FRAC_PI_8;

    //TODO test actual output coordinates

    #[test]
    fn zero_inputs() {
        let radius_range = 1000.0;
        let radius_resolution = 10;
        let longitude_range = [0.0, TWO_PI - FRAC_PI_8, TWO_PI + FRAC_PI_8];
        let longitude_resolution = 5;
        let latitude_range = [0.0, PI - FRAC_PI_8, PI + FRAC_PI_8];
        let latitude_resolution = 5;

        for longitude_resolution in 0..longitude_resolution {
            for longitude_range in longitude_range.iter() {
                for latitude_resolution in 0..latitude_resolution {
                    for latitude_range in latitude_range.iter() {
                        let count = coordinates_count(
                            radius_resolution,
                            *longitude_range,
                            longitude_resolution,
                            *latitude_range,
                            latitude_resolution,
                        );

                        let vector = calculate_coordinates(
                            radius_range,
                            radius_resolution,
                            *longitude_range,
                            longitude_resolution,
                            *latitude_range,
                            latitude_resolution,
                        );

                        let status = count as usize == vector.len();

                        if !status {
                            for (index, point) in vector.iter().enumerate() {
                                println!("{} {:?}", index, point);
                            }
                        }

                        assert_eq!(count as usize, vector.len())
                    }
                }
            }
        }
    }

    #[test]
    fn random_inputs() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        for i in 0..1000000 {
            let radius_range = rng.gen_range(0.0, 1000.0);
            let radius_resolution = rng.gen_range(0, 10);
            let longitude_range = rng.gen_range(0.0, TWO_PI + FRAC_PI_8);
            let longitude_resolution = rng.gen_range(0, 10);
            let latitude_range = rng.gen_range(0.0, PI + FRAC_PI_8);
            let latitude_resolution = rng.gen_range(0, 10);

            let count = coordinates_count(
                radius_resolution,
                longitude_range,
                longitude_resolution,
                latitude_range,
                latitude_resolution,
            );

            let vector = calculate_coordinates(
                radius_range,
                radius_resolution,
                longitude_range,
                longitude_resolution,
                latitude_range,
                latitude_resolution,
            );

            let status = count as usize == vector.len();

            if !status {
                println!(
                    "
                    rep: {}
                    count: {}
                    len: {}
                    radius_range: {}
                    radius_resolution: {}
                    longitude_range: {}
                    longitude_resolution: {}
                    latitude_range: {}
                    latitude_resolution: {}
                    ",
                    i,
                    count,
                    vector.len(),
                    radius_range,
                    radius_resolution,
                    longitude_range,
                    longitude_resolution,
                    latitude_range,
                    latitude_resolution
                );

                for (index, point) in vector.iter().enumerate() {
                    println!("{} {:?}", index, point);
                }
            }

            assert_eq!(count as usize, vector.len())
        }
    }
}
