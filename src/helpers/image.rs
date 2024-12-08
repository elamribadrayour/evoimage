use image::RgbImage;

pub fn load_image(path: &str) -> ([Vec<f64>; 3], u32, u32) {
    log::info!("loading target image");
    let target = image::open(path).unwrap().to_rgb8();
    let mut values: [Vec<f64>; 3] = [
        vec![0.0; target.width() as usize * target.height() as usize],
        vec![0.0; target.width() as usize * target.height() as usize],
        vec![0.0; target.width() as usize * target.height() as usize],
    ];
    target
        .clone()
        .into_raw()
        .iter()
        .enumerate()
        .for_each(|(i, x)| {
            values[i % 3][i / 3] = *x as f64 / 255.0;
        });
    (values, target.width(), target.height())
}

pub fn to_image(array: &[Vec<f64>; 3], width: u32, height: u32) -> RgbImage {
    let mut raw = Vec::with_capacity((width * height * 3) as usize);
    for i in 0..(width * height) as usize {
        raw.push((array[0][i] * 255.0) as u8); // Red
        raw.push((array[1][i] * 255.0) as u8); // Green
        raw.push((array[2][i] * 255.0) as u8); // Blue
    }
    RgbImage::from_vec(width, height, raw).unwrap()
}
