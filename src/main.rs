use output::info;
use std::io;
use std::process;

mod output;
mod settings;

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let mut show_hidden = false;
    let mut sort_by_size = false;
    let mut sort_by_time = false;
    let mut hide_icons = false;

    if args.len() > 1 {
        for arg in args.iter().skip(1) {
            match arg.as_str() {
                "-a" | "--all" => show_hidden = true,

                "-s" | "--size" => sort_by_size = true,

                "-t" | "--time" => sort_by_time = true,

                "-h" | "--hide" => hide_icons = true,

                _ => {
                    println!("\x1b[1;91[Invalid Argument]\x1b[0m\n{}\nFor a list of valid arguments, use \x1b[1mroxo -h\x1b[0m or \x1b[1mroxo --help\x1b[0m", arg);
                }
            }
        }
    }

    if sort_by_time && sort_by_size {
        println!("\x1b[1;91m[Error: Incompatable Arguments]\x1b[0m\n\x1b[1mYou can not sort by time and size at the same time (-s & -t)\x1b[0m\nFor a list of valid arguments, use \x1b[1mroxo -h\x1b[0m or \x1b[1mroxo --help\x1b[0m",);
        process::exit(1);
    }
    let mut file_info_list = info::get_file_info(show_hidden, false, false)?; // Initially unsorted

    if sort_by_size {
        file_info_list.sort_by(|a, b| b.size.cmp(&a.size));
    } else if sort_by_time {
        file_info_list.sort_by(|a, b| a.modified_time.cmp(&b.modified_time));
    } else {
        file_info_list.sort_by(|a, b| {
            let a_is_directory = matches!(a.file_type, info::FileInfoType::Directory);
            let b_is_directory = matches!(b.file_type, info::FileInfoType::Directory);

            if a_is_directory && !b_is_directory {
                return std::cmp::Ordering::Less;
            } else if !a_is_directory && b_is_directory {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Equal
        });
    }

    /*
    println!(
        "\x1b[1m{:<20}\x1b[0m \x1b[1m{:<17}\x1b[0m \x1b[1m{:<14}\x1b[0m",
        "Last Modified", "Size (bytes)", "Name"
    );
    println!("----------------------------------------------------");
    */
    println!(
        "\x1b[1m{:<20}\x1b[0m \x1b[1m{:<14}\x1b[0m",
        "Last Modified", "Name"
    );

    for file_info in &file_info_list {
        let file_type = match file_info.file_type {
            info::FileInfoType::Directory => {
                if !hide_icons {
                    format!("\x1b[1;34m {}\x1b[0m", file_info.name)
                } else {
                    format!("\x1b[1;34m{}\x1b[0m", file_info.name)
                }
            }
            info::FileInfoType::File => {
                if !hide_icons {
                    format!("\x1b[4;31m {}\x1b[0m", file_info.name)
                } else {
                    format!("\x1b[4;31m{}\x1b[0m", file_info.name)
                }
            }
            info::FileInfoType::Symlink => format!("{}", file_info.name),
        };

        /*
        println!(
            "{:<20} {:<17} {:<25}",
            file_info
                .modified_time
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            file_info.size,
            file_type,
        );
        */

        println!(
            "{:<20} {:<25}",
            file_info
                .modified_time
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            file_type,
        );
    }

    Ok(())
}
