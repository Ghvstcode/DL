use clap::{crate_version, Arg, ColorChoice, Command, crate_authors};

pub fn build_app() -> Command {
    let clap_color_choice = if std::env::var_os("NO_COLOR").is_none() {
        ColorChoice::Auto
    } else {
        ColorChoice::Never
    };

    let mut app = Command::new("dl")
        .author(crate_authors!())
        .version(crate_version!())
        .about("Log or journal daily updates on your life/career with ease")
        .color(clap_color_choice)
        .after_help(
            "Note: `dl -h` prints a short and concise overview for more help information run `dl --help`",
        )
        .after_long_help(
            "Bugs can be reported on GitHub: https://github.com/Ghvstcode/dl"
        )
        .arg(
            Arg::new("new")
                .long("new")
                .short('N')
                .help("open a new log file")
                .long_help(
                    // TODO: A better long description
                    "open a new log file",
                ),
        ).arg(
        Arg::new("path")
            .long("path")
            .short('P')
            .help("absolute path to log file location")
            .long_help(
                // TODO: A better long description
                "absolute path to log file location",
            ),
    );

    app
}