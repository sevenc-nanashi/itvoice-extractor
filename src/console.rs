macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        format!("\x1b[38;2;{};{};{}m", $r, $g, $b)
    };
    ($hex:expr) => {
        format!(
            "\x1b[38;2;{};{};{}m",
            $hex >> 16,
            $hex >> 8 & 0xff,
            $hex & 0xff
        )
    };
    () => {
        "\x1b[0m"
    };
}

pub(crate) use rgb;

pub fn show_title() {
    println!(
        "{}== itvoice-extractor ----------------------------------------------------------{}",
        rgb!(0x8232c8),
        rgb!()
    );
    println!(
        "    {}itvoice-extractor / ITVoice解凍ツール{}",
        rgb!(0x8232c8),
        rgb!()
    );
    println!(
        "    Version: {}{}{}",
        rgb!(0x8232c8),
        env!("CARGO_PKG_VERSION"),
        rgb!()
    );
    println!(
        "    Developed by {}名無し｡(@sevenc-nanashi){}",
        rgb!(0x48b0d5),
        rgb!()
    );
    println!("    https://github.com/sevenc-nanashi/itvoice-extractor");
    println!(
        "{}-------------------------------------------------------------------------------{}",
        rgb!(0x8232c8),
        rgb!()
    );
}

pub fn exit_with_pause(code: i32) -> ! {
    press_btn_continue::wait("何かキーを押して終了します。").unwrap();
    std::process::exit(code);
}
