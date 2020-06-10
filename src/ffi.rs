#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn CalculatePointCount1D(radius_resolution: i32, radius_range: i32) -> i32 {
    point_count_1_d(radius_range as u16, radius_resolution as u16) as i32
}

#[no_mangle]
pub extern "C" fn CalculatePointCount2D(
    radius_resolution: i32,
    radius_range: i32,
    longitude_resolution: i32,
    longitude_range: i32,
) -> i32 {
    point_count_2_d(
        radius_range as u16,
        radius_resolution as u16,
        longitude_range as u16,
        longitude_resolution as u16,
    ) as i32
}

#[no_mangle]
pub extern "C" fn CalculatePointCount3D(
    radius_resolution: i32,
    radius_range: i32,
    longitude_resolution: i32,
    longitude_range: i32,
    latitude_resolution: i32,
    latitude_range: i32,
) -> i32 {
    point_count_3_d(
        radius_range as u16,
        radius_resolution as u16,
        longitude_range as u16,
        longitude_resolution as u16,
        latitude_range as u16,
        latitude_resolution as u16,
    );
}

#[no_mangle]
pub extern "C" fn CalculateCoordinates1D(
    radius_resolution: i32,
    radius_range: i32,
    slice_ptr: *mut Coordinates,
    slice_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(slice_ptr, slice_len) };

    calculate_1_d(radius_range as u16, radius_resolution as u16, slice);
}

#[no_mangle]
pub extern "C" fn CalculateCoordinates2D(
    radius_resolution: i32,
    radius_range: i32,
    longitude_resolution: i32,
    longitude_range: i32,
    slice_ptr: *mut Coordinates,
    slice_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(slice_ptr, slice_len) };

    calculate_2_d(
        radius_range as u16,
        radius_resolution as u16,
        longitude_range as u16,
        longitude_resolution as u16,
        slice,
    );
}

#[no_mangle]
pub extern "C" fn CalculateCoordinates3D(
    radius_resolution: i32,
    radius_range: i32,
    longitude_resolution: i32,
    longitude_range: i32,
    latitude_resolution: i32,
    latitude_range: i32,
    slice_ptr: *mut Coordinates,
    slice_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(slice_ptr, slice_len) };

    calculate_3_d(
        radius_range as u16,
        radius_resolution as u16,
        longitude_range as u16,
        longitude_resolution as u16,
        latitude_range as u16,
        latitude_resolution as u16,
        slice,
    );
}
