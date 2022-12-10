use std::{env, path::PathBuf};

use crate::console::{exit_with_pause, rgb};

pub fn run() -> Vec<PathBuf> {
    let file_path = env::args().nth(1).unwrap_or_else(||{
        println!(
            "itvoice-extractor.exeにITVOICE-v{}x.x.x{}_.zip.001_.zipをドラッグ&ドロップしてください。",
            rgb!(0x8232c8),
            rgb!()
        );
        exit_with_pause(1);
    });
    if !file_path.ends_with(".001_.zip") {
        println!(
            "itvoice-extractor.exeにITVOICE-v{}x.x.x{}_.zip.001_.zipをドラッグ&ドロップしてください。",
            rgb!(0x8232c8),
            rgb!()
        );
        exit_with_pause(1);
    }
    let file_name_regex = regex::Regex::new(r"ITVOICE-v\d+\.\d+\.\d+_.zip.(\d+)_\.zip").unwrap();

    let files = glob::glob(&format!("{}.*_.zip", file_path.replace(".001_.zip", "")))
        .unwrap()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let last_file_index = files
        .iter()
        .map(|x| {
            let file_name = x.file_name().unwrap().to_str().unwrap();
            let caps = file_name_regex.captures(file_name).unwrap();
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap()
        })
        .max()
        .unwrap();
    if files.len() != last_file_index {
        println!("途中のファイルが抜け落ちています。");
        exit_with_pause(1);
    }
    files
}
