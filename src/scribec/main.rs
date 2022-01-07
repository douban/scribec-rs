use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "scribe client commands")]
struct ClientArgs {
    #[argh(subcommand)]
    subcommand: ScribeSubCommandEnum,

    #[argh(
        option,
        short = 'H',
        default = "String::from(\"localhost\")",
        description = "default: localhost",
    )]
    host: String,

    #[argh(option, short = 'p',default = "1463", description = "default: 1463")]
    port: u32,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum ScribeSubCommandEnum {
    Cat(SubCommandCat),
    StdLogger(SubCommandStdLogger),
    Admin(SubCommandAdmin),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "cat", description = "cat")]
struct SubCommandCat {
    #[argh(positional)]
    category: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "std_logger", description = "std_logger")]
struct SubCommandStdLogger {
    #[argh(positional)]
    category: String,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "admin", description = "admin")]
struct SubCommandAdmin {
    #[argh(positional)]
    command: String,
}



fn main() {
    let args: ClientArgs = argh::from_env();
    println!("port={}", args.port);
    println!("host={}", args.host);

    match args.subcommand {
        ScribeSubCommandEnum::Cat(cat) => {
            println!("scribe cat: category={}", cat.category);
        }
        ScribeSubCommandEnum::StdLogger(std_logger) => {
            println!("scribe std_logger: category={}", std_logger.category);
        }
        ScribeSubCommandEnum::Admin(admin) => {
            println!("scribe admin: command={}", admin.command);
        }
    }
}
