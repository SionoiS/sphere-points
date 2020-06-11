#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn CoordinatesCount(
    radius_resolution: i32,
    longitude_resolution: i32,
    longitude_range: f32,
    latitude_resolution: i32,
    latitude_range: f32,
) -> i32 {
    coordinates_count(
        radius_resolution,
        longitude_range,
        longitude_resolution,
        latitude_range,
        latitude_resolution,
    );
}

#[no_mangle]
pub extern "C" fn CalculateCoordinates(
    radius_resolution: i32,
    radius_range: f32,
    longitude_resolution: i32,
    longitude_range: f32,
    latitude_resolution: i32,
    latitude_range: f32,
    slice_ptr: *mut Coordinates,
    slice_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(slice_ptr, slice_len) };

    calculate_coordinates_inplace(
        radius_range,
        radius_resolution,
        longitude_range,
        longitude_resolution,
        latitude_range,
        latitude_resolution,
        slice,
    );
}
