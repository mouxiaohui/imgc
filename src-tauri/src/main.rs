#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod color;

use color::Colors;

use image::imageops::FilterType;
use kmeans_colors::get_kmeans_hamerly;
use palette::{Lab, Pixel, Srgb, Srgba};

#[allow(dead_code)]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![extraction_color])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn extraction_color(image_base64: String) -> Result<Vec<Colors>, String> {
    match base64::decode(&image_base64) {
        Ok(bytes) => {
            let colors;
            if let Ok(resized) = resize_image(&bytes, 200, 200) {
                colors = get_icolors_from(&resized);
            } else {
                colors = get_icolors_from(&bytes);
            };

            Ok(colors)
        }
        Err(err) => Err(err.to_string()),
    }
}

fn resize_image(bytes: &Vec<u8>, nwidth: u32, nheight: u32) -> Result<Vec<u8>, String> {
    match image::load_from_memory(&bytes) {
        Ok(d_image) => {
            let resized_image = d_image.resize(nwidth, nheight, FilterType::Nearest);
            Ok(resized_image.into_rgba8().into_raw())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_icolors_from(img_bytes: &Vec<u8>) -> Vec<Colors> {
    let lab: Vec<Lab> = Srgba::from_raw_slice(img_bytes)
        .iter()
        .map(|x| x.into_format().into())
        .collect();

    let result = get_kmeans_hamerly(10, 20, 1.0, false, &lab, 0);

    let rgb = &result
        .centroids
        .iter()
        .map(|x| Srgb::from(*x).into_format())
        .collect::<Vec<Srgb<u8>>>();

    let mut colors = Vec::new();
    for c in rgb {
        let mut ple = Colors::new((c.red, c.green, c.blue));
        ple.generate_hex();
        colors.push(ple);
    }

    colors
}
