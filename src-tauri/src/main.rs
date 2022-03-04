#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod color;

use color::IColor;

use image::imageops::FilterType;
use kmeans_colors::get_kmeans_hamerly;
use palette::{Lab, Pixel, Srgb, Srgba};

#[allow(dead_code)]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_palette])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_palette(path: String) -> Result<Vec<IColor>, String> {
    match image::open(path) {
        Ok(img) => {
            let img_bytes = img
                .resize(400, 400, FilterType::Nearest)
                .into_rgba8()
                .into_raw();

            Ok(get_colors_from(img_bytes))
        }
        Err(err) => Err(err.to_string()),
    }
}

fn get_colors_from(img_bytes: Vec<u8>) -> Vec<IColor>{
    let lab: Vec<Lab> = Srgba::from_raw_slice(&img_bytes)
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

    let mut icolors = Vec::new();
    for c in rgb {
        let mut icolor = IColor::new((c.red, c.green, c.blue));
        icolor.generate_hex();
        icolors.push(icolor);
    }

    icolors
}
