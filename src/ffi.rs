#![allow(non_snake_case)]

#[no_mangle]
pub extern "C" fn Calculate3D(
    radius_resolution: i32,
    radius_range: i32,
    longitude_resolution: i32,
    radius_range: i32,
    latitude_resolution: i32,
    latitude_range: i32,
    slice_ptr: *mut Coordinates,
    slice_len: usize,
) {
    let slice = unsafe { std::slice::from_raw_parts(slice_ptr, slice_len) };

    calculate_3_d(10, 1, 90, 15, 90, 15, slice);
}

//TODO make it available to external code... also test it
