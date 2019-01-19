extern crate clap;
extern crate db;
extern crate front;
use clap::{App, Arg, SubCommand};
use db::docker::ImageDrive;

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .help("Sets an optional output file")
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about("list entries or items")
                .arg(
                    Arg::with_name("entry")
                        .help("set it if list items")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("put")
                .about("put host file to imagedrive")
                .arg(Arg::with_name("entry").help("entry name").required(true))
                .arg(Arg::with_name("file").help("file path").required(true)),
        )
        .subcommand(
            SubCommand::with_name("export")
                .about("export entry to host")
                .arg(Arg::with_name("entry").help("entry name").required(true))
                .arg(Arg::with_name("dir").help("dst dir path").required(true)),
        )
        .get_matches();
    // TODO: config file

    let username = "rcmerci";
    let server = "registry.cn-hangzhou.aliyuncs.com";
    let password = "1Qazse4r+9s";
    let image_name = "registry.cn-hangzhou.aliyuncs.com/wangpan/test";

    if let Some(matches) = matches.subcommand_matches("ls") {
        // "$ myapp test" was run
        if matches.is_present("entry") {
            front::list_entry_item(
                &ImageDrive::new(image_name, server, username, password),
                matches.value_of("entry").unwrap(),
            );
        } else {
            front::list_entry(&ImageDrive::new(image_name, server, username, password));
        }
    }

    if let Some(matches) = matches.subcommand_matches("put") {
        let entry = matches.value_of("entry").unwrap();
        let filepath = matches.value_of("file").unwrap();
        front::put(
            &ImageDrive::new(image_name, server, username, password),
            entry,
            filepath,
        );
    }

    if let Some(matches) = matches.subcommand_matches("export") {
        let entry = matches.value_of("entry").unwrap();
        let filepath = matches.value_of("dir").unwrap();
        front::export(
            &ImageDrive::new(image_name, server, username, password),
            entry,
            filepath,
        );
    }

    if let Some(matches) = matches.subcommand_matches("sync") {}

    // Continued program logic goes here...
}
// extern crate clap;
// extern crate front;

// use clap::{App, Arg, SubCommand};
// fn main() {
//     let matches = App::new("ImageDrive")
//         .version("1.0")
//         .author("zj")
//         .arg(
//             Arg::with_name("config")
//                 .short("c")
//                 .long("config")
//                 .value_name("FILE")
//                 .help("Sets a custom config file")
//                 .takes_value(true),
//         )
//         .arg(
//             Arg::with_name("INPUT")
//                 .help("Sets the input file to use")
//                 .required(false)
//                 .index(1),
//         )
//         .arg(
//             Arg::with_name("v")
//                 .short("v")
//                 .multiple(true)
//                 .help("Sets the level of verbosity"),
//         )
//         .subcommand(
//             SubCommand::with_name("ls")
//                 .about("list entries or items")
//                 .arg(Arg::with_name("ENTRY").index(1).required(false)),
//         )
//         .get_matches();
//     matches.usage
//     // println!("{:?}", matches);
// }
