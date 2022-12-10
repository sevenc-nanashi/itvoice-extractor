use std::{
    fs, io,
    io::{Read, Write},
    path::PathBuf,
    sync::mpsc,
    thread,
};

use crate::console::rgb;

struct Progress {
    index: usize,
    progress: f64,
}

pub fn run(files: &Vec<PathBuf>, temp_dir: &tempfile::TempDir) {
    let mut threads = vec![];

    println!(
        "一次解凍先：{}{}{}",
        rgb!(0x8232c8),
        temp_dir.path().display(),
        rgb!()
    );

    println!("一次解凍中...");
    let (tx, rx) = mpsc::channel();
    for (i, entry) in files.iter().enumerate() {
        let file = fs::File::open(&entry).unwrap();
        let l_temp_dir = temp_dir.path().to_path_buf();
        let ltx = tx.clone();
        threads.push(thread::spawn(move || {
            let mut archive = zip::ZipArchive::new(file).unwrap();
            let mut main_file = archive.by_index(0).unwrap();
            let mut file = fs::File::create(l_temp_dir.join(main_file.name())).unwrap();
            let mut current_pos = 0;
            let mut buf = vec![0; 1024 * 1024];
            loop {
                let read_size = main_file.read(&mut buf).unwrap();
                if read_size == 0 {
                    break;
                }
                file.write_all(&buf[..read_size]).unwrap();
                current_pos += read_size;
                ltx.send(Progress {
                    index: i,
                    progress: current_pos as f64 / main_file.size() as f64,
                })
                .unwrap();
            }

            ltx.send(Progress {
                index: i,
                progress: 1.0,
            })
            .unwrap();
        }));
    }
    let files_len = files.len();
    let renderer = thread::spawn(move || {
        let out = io::stdout();
        let mut out = out.lock();
        let mut progress = vec![0.0; files_len];
        loop {
            let p = rx.recv().unwrap();
            progress[p.index] = p.progress;
            let progress_str = progress
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    format!(
                        "{}{}{:0>3}：{}{: >3}%{}",
                        if i == 0 { "  " } else { " | " },
                        if p == &1.0 {
                            rgb!(0x8232c8)
                        } else {
                            "".to_string()
                        },
                        i + 1,
                        rgb!(0x8232c8),
                        (p * 100.0) as usize,
                        rgb!()
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            write!(out, "{}\r", progress_str).unwrap();
            if progress.iter().all(|x| x == &1.0) {
                break;
            }
        }
    });
    for thread in threads {
        thread.join().unwrap();
    }
    renderer.join().unwrap();
    println!("\n{}! 一次解凍完了。\n{}", rgb!(0x8232c8), rgb!());
}
