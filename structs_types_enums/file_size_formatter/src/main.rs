fn main() {
    // read the file size from the command line argument
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    match args.len() {
        3 => {
            let size: u64 = args[1].parse().expect("Please provide a valid file size");
            let unit = &args[2];
            match unit.as_str() {
                "B" | "KB" | "MB" | "GB" | "TB" => {
                    let formatted_size = get_filesize(size, unit);
                    println!("{:?}", formatted_size);
                },
                _ => {
                    eprintln!("Invalid size unit. Please use B, KB, MB, GB, or TB.");
                    std::process::exit(1);
                }
            }
        },
        _ => {
            eprintln!(
                "Usage: file_size_formatter <file_size> <size_unit> (e.g. 10 KB, 5 MB, 1 GB)"
            );
            std::process::exit(1);
        }
    }
}

#[derive(Debug)]
struct Sizes {
    bytes: u64,
    kilobytes: u64,
    megabytes: u64,
    gigabytes: u64,
    terabytes: u64,
}

fn get_filesize(size: u64, unit: &str) -> Sizes {
    match unit {
        "B" => Sizes {
            bytes: size,
            kilobytes: size / 1000,
            megabytes: size / 1_000_000,
            gigabytes: size / 1_000_000_000,
            terabytes: size / 1_000_000_000_000,
        },
        "KB" => Sizes {
            bytes: size * 1000,
            kilobytes: size,
            megabytes: size / 1000,
            gigabytes: size / 1_000_000,
            terabytes: size / 1_000_000_000,
        },
        "MB" => Sizes {
            bytes: size * 1_000_000,
            kilobytes: size * 1000,
            megabytes: size,
            gigabytes: size / 1000,
            terabytes: size / 1_000_000,
        },
        "GB" => Sizes {
            bytes: size * 1_000_000_000,
            kilobytes: size * 1_000_000,
            megabytes: size * 1000,
            gigabytes: size,
            terabytes: size / 1000,
        },
        "TB" => Sizes {
            bytes: size * 1_000_000_000_000,
            kilobytes: size * 1_000_000_000,
            megabytes: size * 1_000_000,
            gigabytes: size * 1000,
            terabytes: size,
        },
        _ => panic!("Invalid unit"),
    }
}
