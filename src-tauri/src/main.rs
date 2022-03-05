#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod color;

use color::Palette;

use image::imageops::FilterType;
use kmeans_colors::get_kmeans_hamerly;
use palette::{Lab, Pixel, Srgb, Srgba};

#[allow(dead_code)]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_palettes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_palettes(image_base64: String) -> Result<Vec<Palette>, String> {
    match base64::decode(&image_base64) {
        Ok(bytes) => {
            let palettes;
            if let Ok(resized) = resize_image(&bytes, 300, 300) {
                palettes = get_icolors_from(&resized);
            } else {
                palettes = get_icolors_from(&bytes);
            };

            Ok(palettes)
        }
        Err(err) => Err(err.to_string()),
    }
}

fn resize_image(bytes: &Vec<u8>, nwidth: u32, nheight: u32) -> Result<Vec<u8>, String> {
    match image::load_from_memory(&bytes) {
        Ok(d_image) => {
            let resized_image = d_image.resize(nwidth, nheight, FilterType::Nearest);
            Ok(resized_image.into_rgb8().into_raw())
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_icolors_from(img_bytes: &Vec<u8>) -> Vec<Palette> {
    let lab: Vec<Lab> = Srgba::from_raw_slice(img_bytes)
        .iter()
        .map(|x| x.into_format().into())
        .collect();

    let max_iterations = 20;
    let converge = 1.0;
    let verbose = false;
    let seed: u64 = 0;

    let result = get_kmeans_hamerly(10, max_iterations, converge, verbose, &lab, seed);

    let rgb = &result
        .centroids
        .iter()
        .map(|x| Srgb::from(*x).into_format())
        .collect::<Vec<Srgb<u8>>>();

    let mut palettes = Vec::new();
    for c in rgb {
        let mut ple = Palette::new((c.red, c.green, c.blue));
        ple.generate_hex();
        palettes.push(ple);
    }

    palettes
}
