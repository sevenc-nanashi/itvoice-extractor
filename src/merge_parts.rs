use std::{
    fs, io,
    io::{Read, Write},
    path::PathBuf,
};

use crate::console::rgb;

pub fn run(temp_dir: &tempfile::TempDir) -> PathBuf {
    let out = io::stdout();
    let mut out = out.lock();

    println!("結合中...");
    let mut output_files = glob::glob(&format!(
        "{}/ITVOICE-v*_.zip.*",
        temp_dir.path().to_str().unwrap()
    ))
    .unwrap()
    .map(|x| x.unwrap())
    .collect::<Vec<_>>();
    println!(
        "  {}{}個{}のファイルを結合します。",
        rgb!(0x8232c8),
        output_files.len(),
        rgb!()
    );
    output_files.sort_by(|a, b| {
        let a = a.file_name().unwrap().to_str().unwrap();
        let b = b.file_name().unwrap().to_str().unwrap();
        a.cmp(b)
    });
    let concat_file_path = temp_dir.path().join("final.zip");
    let mut concat_file = fs::File::create(&concat_file_path).unwrap();
    let total_size = output_files
        .iter()
        .map(|x| fs::metadata(x).unwrap().len())
        .sum::<u64>();

    let mut current_pos = 0;

    let mut buf = vec![0; 1024 * 1024];
    for (i, entry) in output_files.iter().enumerate() {
        let mut file = fs::File::open(&entry).unwrap();
        loop {
            let read_size = file.read(&mut buf).unwrap();
            if read_size == 0 {
                break;
            }
            concat_file.write_all(&buf[..read_size]).unwrap();
            current_pos += read_size;
            write!(
                out,
                "{}",
                format!(
                    "  {}{: >3}%{} ({:0>3}/{:0>3})\r",
                    rgb!(0x8232c8),
                    (current_pos as f64 / total_size as f64 * 100.0) as usize,
                    rgb!(),
                    i + 1,
                    output_files.len()
                )
            )
            .unwrap();
        }
    }
    println!("\n{}! 結合完了。\n{}", rgb!(0x8232c8), rgb!());
    concat_file_path
}
