mod console;
mod extract_merged;
mod extract_parts;
mod merge_parts;
mod validate_files;
use console::{exit_with_pause, rgb, show_title};
use std::process::Command;

fn main() {
    enable_ansi_support::enable_ansi_support().unwrap_or_else(|_| {
        eprintln!("ANSIIコードのサポートを有効にできませんでした。");
        exit_with_pause(1);
    });
    show_title();

    let temp_dir = tempfile::tempdir().unwrap();

    let files = validate_files::run();

    extract_parts::run(&files, &temp_dir);

    let concat_file_path = merge_parts::run(&temp_dir);

    let final_dest = extract_merged::run(&concat_file_path);
    drop(temp_dir);

    println!(
        "{}解凍に成功しました！{}\n  出力先：{}{}{}",
        rgb!(0x8232c8),
        rgb!(),
        rgb!(0x8232c8),
        final_dest.replace('/', "\\"),
        rgb!()
    );

    Command::new("explorer")
        .arg(final_dest.replace('/', "\\"))
        .spawn()
        .unwrap();

    exit_with_pause(0)
}
