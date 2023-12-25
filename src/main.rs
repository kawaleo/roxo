use prettytable::{format, row, Cell, Row, Table};
use std::io;

mod file_info;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut show_hidden = false;
    let mut sort_by_size = false;
    let mut sort_by_time = false;

    // Parse command line arguments
    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-a" | "--all" => show_hidden = true,
                "-s" | "--size" => sort_by_size = true,
                "-t" | "--time" => sort_by_time = true,
                _ => println!("Unknown option: {}", arg),
            }
        }
    }

    let mut file_info_list = file_info::get_file_info(show_hidden, sort_by_size, sort_by_time)?;

    // Sort file_info_list to display directories first, then files
    file_info_list.sort_by(|a, b| {
        let a_is_directory = matches!(a.file_type, file_info::FileInfoType::Directory);
        let b_is_directory = matches!(b.file_type, file_info::FileInfoType::Directory);

        if a_is_directory && !b_is_directory {
            return std::cmp::Ordering::Less;
        } else if !a_is_directory && b_is_directory {
            return std::cmp::Ordering::Greater;
        }
        std::cmp::Ordering::Equal
    });

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(row![
        "\x1b[1m\x1b[4mName\x1b[0m",
        "\x1b[1m\x1b[4mSize (bytes)\x1b[0m",
        "\x1b[1m\x1b[4mLast Modified\x1b[0m"
    ]);

    for file_info in file_info_list {
        let file_type = match file_info.file_type {
            file_info::FileInfoType::Directory => {
                format!("\x1b[1;34m{}\x1b[0m", file_info.name) // Light blue and bold for directories
            }
            file_info::FileInfoType::File => {
                format!("\x1b[4;31m{}\x1b[0m", file_info.name) // Light red and underlined for files
            }
            file_info::FileInfoType::Symlink => file_info.name,
            // Handle other types as needed
        };

        table.add_row(Row::new(vec![
            Cell::new(&file_type),
            Cell::new(&file_info.size.to_string()),
            Cell::new(
                &file_info
                    .modified_time
                    .format("%Y-%m-%d %H:%M:%S")
                    .to_string(),
            ),
        ]));
    }

    table.printstd();

    Ok(())
}
