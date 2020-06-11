use std::f32::consts::{FRAC_PI_2, PI};

const TWO_PI: f32 = std::f32::consts::PI * 2.0;

/// Calculate the number of coordinates.
pub fn coordinates_count(
    radius_resolution: i32,
    longitude_range: f32,
    mut longitude_resolution: i32,
    latitude_range: f32,
    mut latitude_resolution: i32,
) -> i32 {
    if radius_resolution == 0 {
        return 1;
    }

    if longitude_range == 0.0 {
        return 1 + radius_resolution;
    }

    let longitude_range = longitude_range.min(TWO_PI);
    let longitude_range = longitude_range.max(0.0);

    longitude_resolution += 1;

    if (longitude_range - TWO_PI).abs() < 0.0001 {
        // remove duplicate meridian
        longitude_resolution -= 1;
    }

    if latitude_range == 0.0 {
        return 1 + radius_resolution * longitude_resolution;
    }

    let latitude_range = latitude_range.min(PI);
    let latitude_range = latitude_range.max(0.0);

    latitude_resolution += 1;

    if (latitude_range - PI).abs() < 0.0001 {
        // remove duplicate poles
        latitude_resolution -= 2;
    }

    if latitude_resolution < 0 {
        latitude_resolution = 0;
    }

    1 + radius_resolution * longitude_resolution * latitude_resolution
}

pub fn calculate_coordinates(
    radius_range: f32,
    radius_resolution: i32,
    longitude_range: f32,
    longitude_resolution: i32,
    latitude_range: f32,
    latitude_resolution: i32,
    //slice: &mut [Coordinates],
) -> Vec<Coordinates> {
    let radius_step = radius_range / radius_resolution as f32;
    //println!("radius_step: {}", radius_step);

    let longitude_range = longitude_range.min(TWO_PI);
    let longitude_range = longitude_range.max(0.0);
    //println!("longitude_range: {}", longitude_range);

    let mut longitude_step = longitude_range / longitude_resolution as f32;
    if !longitude_step.is_normal() {
        longitude_step = 0.0;
    }
    //println!("longitude_step: {}", longitude_step);

    let longitude_offset = longitude_range / 2.0;
    //println!("longitude_offset: {}", longitude_offset);

    let latitude_range = latitude_range.min(PI);
    let latitude_range = latitude_range.max(0.0);
    //println!("latitude_range: {}", latitude_range);

    let mut latitude_step = latitude_range / latitude_resolution as f32;
    if !latitude_step.is_normal() {
        latitude_step = 0.0;
    }
    //println!("latitude_step: {}", latitude_step);

    let latitude_offset = latitude_range / 2.0;
    //println!("latitude_offset: {}", latitude_offset);

    let mut vector = Vec::with_capacity(100);

    vector.push(Coordinates {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });

    //let mut index = 1;

    //Step 3 radius
    for i in 1..radius_resolution + 1 {
        if radius_resolution < 1 {
            return vector;
        }

        let radius = radius_step * i as f32;

        //Step 2 latitude
        for j in 0..latitude_resolution + 1 {
            //if full latitude
            if (latitude_range - PI).abs() < 0.0001 {
                //skip north pole
                if j == 0 {
                    continue;
                }

                //south pole
                if j == latitude_resolution {
                    break;
                }
            }

            let phi = latitude_offset - latitude_step * j as f32;

            //Step 1 longitude
            for k in 0..longitude_resolution + 1 {
                // skip duplicate meridian
                if (longitude_range - TWO_PI).abs() < 0.0001 && k == longitude_resolution {
                    break;
                }

                let theta = longitude_offset - longitude_step * k as f32;

                /* println!(
                    "Spherical {:?}",
                    Coordinates {
                        x: radius,
                        y: phi,
                        z: theta
                    }
                ); */

                vector.push(spherical_to_cartesian(
                    radius,
                    theta + FRAC_PI_2,
                    phi + FRAC_PI_2,
                ));
                /* slice[index] = spherical_to_cartesian(radius, theta + FRAC_PI_2, phi + FRAC_PI_2);
                index += 1; */
            }
        }
    }

    vector
}

pub fn calculate_coordinates_inplace(
    radius_range: f32,
    radius_resolution: i32,
    longitude_range: f32,
    longitude_resolution: i32,
    latitude_range: f32,
    latitude_resolution: i32,
    slice: &mut [Coordinates],
) {
    let radius_step = radius_range / radius_resolution as f32;
    //println!("radius_step: {}", radius_step);

    let longitude_range = longitude_range.min(TWO_PI);
    let longitude_range = longitude_range.max(0.0);
    //println!("longitude_range: {}", longitude_range);

    let mut longitude_step = longitude_range / longitude_resolution as f32;
    if !longitude_step.is_normal() {
        longitude_step = 0.0;
    }
    //println!("longitude_step: {}", longitude_step);

    let longitude_offset = longitude_range / 2.0;
    //println!("longitude_offset: {}", longitude_offset);

    let latitude_range = latitude_range.min(PI);
    let latitude_range = latitude_range.max(0.0);
    //println!("latitude_range: {}", latitude_range);

    let mut latitude_step = latitude_range / latitude_resolution as f32;
    if !latitude_step.is_normal() {
        latitude_step = 0.0;
    }
    //println!("latitude_step: {}", latitude_step);

    let latitude_offset = latitude_range / 2.0;
    //println!("latitude_offset: {}", latitude_offset);

    slice[0] = Coordinates {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let mut index = 1;

    //Step 3 radius
    for i in 1..radius_resolution + 1 {
        if radius_resolution < 1 {
            return;
        }

        let radius = radius_step * i as f32;

        //Step 2 latitude
        for j in 0..latitude_resolution + 1 {
            //if full latitude
            if (latitude_range - PI).abs() < 0.0001 {
                //skip north pole
                if j == 0 {
                    continue;
                }

                //south pole
                if j == latitude_resolution {
                    break;
                }
            }

            let phi = latitude_offset - latitude_step * j as f32;

            //Step 1 longitude
            for k in 0..longitude_resolution + 1 {
                // skip duplicate meridian
                if (longitude_range - TWO_PI).abs() < 0.0001 && k == longitude_resolution {
                    break;
                }

                let theta = longitude_offset - longitude_step * k as f32;

                /* println!(
                    "Spherical {:?}",
                    Coordinates {
                        x: radius,
                        y: phi,
                        z: theta
                    }
                ); */

                slice[index] = spherical_to_cartesian(radius, theta + FRAC_PI_2, phi + FRAC_PI_2);
                index += 1;
            }
        }
    }
}

fn spherical_to_cartesian(radius: f32, theta: f32, phi: f32) -> Coordinates {
    let (sin_phi, cos_phi) = phi.sin_cos();
    let (sin_theta, cos_theta) = theta.sin_cos();

    //let x = radius * sin_phi * theta.cos();
    //let y = radius * sin_phi * theta.sin();
    //let z = radius * phi.cos();
    // Axis Y and Z switched from math notation to something useable in game.
    // Normally in games Z is forward and Y is up.
    Coordinates {
        x: radius * sin_phi * cos_theta,
        y: radius * cos_phi,
        z: radius * sin_phi * sin_theta,
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct Coordinates {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Distribution, Uniform};

    //TODO test actual output coordinates

    #[test]
    fn random_inputs() {
        let mut rng = rand::thread_rng();

        let radius_range = Uniform::new_inclusive(0.0, 1000.0);
        let radius_resolution = Uniform::new_inclusive(0, 10);
        let longitude_range = Uniform::new_inclusive(0.0, TWO_PI + PI);
        let longitude_resolution = Uniform::new_inclusive(0, 10);
        let latitude_range = Uniform::new_inclusive(0.0, TWO_PI);
        let latitude_resolution = Uniform::new_inclusive(0, 10);

        for i in 0..1000000 {
            let radius_range = radius_range.sample(&mut rng);
            let radius_resolution = radius_resolution.sample(&mut rng);
            let longitude_range = longitude_range.sample(&mut rng);
            let longitude_resolution = longitude_resolution.sample(&mut rng);
            let latitude_range = latitude_range.sample(&mut rng);
            let latitude_resolution = latitude_resolution.sample(&mut rng);

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

            assert_eq!(status, true);
        }
    }

    #[test]
    fn random_inputs_inplace() {
        let mut rng = rand::thread_rng();

        let radius_range = Uniform::new_inclusive(0.0, 1000.0);
        let radius_resolution = Uniform::new_inclusive(0, 10);
        let longitude_range = Uniform::new_inclusive(0.0, TWO_PI + PI);
        let longitude_resolution = Uniform::new_inclusive(0, 10);
        let latitude_range = Uniform::new_inclusive(0.0, TWO_PI);
        let latitude_resolution = Uniform::new_inclusive(0, 10);

        for i in 0..1000000 {
            let radius_range = radius_range.sample(&mut rng);
            let radius_resolution = radius_resolution.sample(&mut rng);
            let longitude_range = longitude_range.sample(&mut rng);
            let longitude_resolution = longitude_resolution.sample(&mut rng);
            let latitude_range = latitude_range.sample(&mut rng);
            let latitude_resolution = latitude_resolution.sample(&mut rng);

            let count = coordinates_count(
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

            calculate_coordinates_inplace(
                radius_range,
                radius_resolution,
                longitude_range,
                longitude_resolution,
                latitude_range,
                latitude_resolution,
                &mut vector,
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

            assert_eq!(status, true);
        }
    }
}
