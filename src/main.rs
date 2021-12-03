use aoc21::{run_day, Part};
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("Advent Of Code 2021")
        .author("LeMoonStar <webmaster@unitcore.de>")
        .about("My Advent Of Code 2021 solutions.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("day")
                .help("The number of the day to execute")
                .required(true)
                .takes_value(true)
                .validator(|v| match v.parse::<u8>() {
                    Ok(day) => {
                        if 0 < day && day <= 25 {
                            Ok(())
                        } else {
                            Err("The day must be between 1 and 25.".to_string())
                        }
                    }
                    Err(_) => Err("The day must be a number between 1 and 25.".to_string()),
                }),
        )
        .arg(
            Arg::with_name("part")
                .help("Specifies the part of the day to compute.")
                .long("part")
                .short("p")
                .default_value("b")
                .possible_values(&["1", "2", "b"])
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Print verbose information")
                .long("verbose")
                .short("v"))
        .subcommand(
            SubCommand::with_name("test").about("Test the day with the example input data."),
        )
        .subcommand(
            SubCommand::with_name("auto")
                .about("Automatically download input from AoC using the provided session and run the solution.")
                .arg(Arg::with_name("session")
                    .help("The AoC browser session string. If not provided, uses the AOC_SESSION eviroment variable.")
                    .short("s")
                    .long("session")
                    .takes_value(true))
                .arg(Arg::with_name("no_cache")
                    .help("Dont cache the input, and delete any current cache for this day.")
                    .short("N")
                    .long("no-cache")))
        .subcommand(
            SubCommand::with_name("run")
                .about("Use either a file or stdin as input and run the solution.")
                .arg(Arg::with_name("file")
                    .help("Specify a file to be used as input, otherwise use stdin.")
                    .short("f")
                    .long("file")
                    .takes_value(true)
            )
        )
        .get_matches();

    println!("{:?}", matches);

    match matches.subcommand_matches("test") {
        Some(_) => {}
        None => {}
    }
    run_day(1, Part::Both, &"1".to_owned());
}
