use std::{
    env, fs, io,
    io::Write,
    path::{Path, PathBuf},
};

use crate::console::rgb;

pub fn run(concat_file_path: &PathBuf) -> String {
    let out = io::stdout();
    let mut out = out.lock();

    let final_dest = env::var("LOCALAPPDATA").unwrap() + "/programs";

    let concat_file = fs::File::open(&concat_file_path).unwrap();
    let mut archive = zip::ZipArchive::new(concat_file).unwrap();

    let root_dir = {
        let first_file = archive.by_index(0).unwrap();
        first_file.name().split('/').next().unwrap().to_string()
    };

    println!(
        "解凍先：{}{}{}",
        rgb!(0x8232c8),
        (final_dest.clone() + "/itvoice").replace('/', "\\"),
        rgb!()
    );
    fs::create_dir_all(&final_dest).unwrap();
    println!("解凍中...");
    let total_files = archive.len();
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        let outpath_str =
            final_dest.clone() + "/" + &outpath.to_string_lossy().replace(&root_dir, "itvoice");
        let outpath = Path::new(&outpath_str);

        write!(
            out,
            "{}",
            format!(
                "\x1b[2K  {}{:>6.2}%{} ({: >width$}/{}) {}\r",
                rgb!(0x8232c8),
                i as f64 / total_files as f64 * 100.0,
                rgb!(),
                i + 1,
                total_files,
                outpath.file_name().unwrap().to_str().unwrap(),
                width = total_files.to_string().len(),
            )
        )
        .unwrap();
        io::stdout().flush().unwrap();
        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
    println!(
        "\x1b[2K  {}100.00%{} ({}/{})",
        rgb!(0x8232c8),
        rgb!(),
        total_files,
        total_files
    );
    println!("{}! 解凍完了。\n{}", rgb!(0x8232c8), rgb!());
    final_dest + "\\itvoice"
}
