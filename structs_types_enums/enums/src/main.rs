enum FileSize {
    Bytes(u64, String),
    Kilobytes(u64, String),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size, "test".to_string()),
        1000..=999_999 => FileSize::Kilobytes(size / 1000, "test".to_string()),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_000_000_000 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000)
    };

    match filesize {
        FileSize::Bytes(bytes, _) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb, _) => format!("{:.2} KB", kb as f64),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64),
        FileSize::Terabytes(gb) => format!("{:.2} TB", gb as f64),
    }
}


fn main() {
    let result = format_size(6556882337399);
    println!("{}", result)
}
