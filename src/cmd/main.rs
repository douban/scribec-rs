use std::io;

use argh::FromArgs;

use scribec::fb303::Fb303Status;
use scribec::scribe::LogEntry;
use scribec::client::ScribeClient;

#[derive(FromArgs, PartialEq, Debug)]
#[argh(description = "scribe client commands")]
struct ClientArgs {
    #[argh(subcommand)]
    subcommand: ScribeSubCommandEnum,

    #[argh(
        option,
        short = 'H',
        default = "String::from(\"localhost\")",
        description = "default: localhost"
    )]
    host: String,

    #[argh(option, short = 'p', default = "1463", description = "default: 1463")]
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
    let mut client = ScribeClient::new(args.host, args.port);

    match args.subcommand {
        ScribeSubCommandEnum::Cat(cat) => {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let logs = vec![LogEntry {
                category: Some(cat.category.clone()),
                message: Some(buffer),
            }];
            client.log(logs).unwrap();
        }
        ScribeSubCommandEnum::StdLogger(std_logger) => loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            let logs = vec![LogEntry {
                category: Some(std_logger.category.clone()),
                message: Some(buffer),
            }];
            client.log(logs).unwrap();
        },
        ScribeSubCommandEnum::Admin(admin) => match admin.command.as_str() {
            "alive" => {
                let res = client.alive_since().unwrap();
                println!("{}", res);
            }
            "status" => {
                let mut msg = String::new();
                let res = client.get_status().unwrap();
                match res {
                    Fb303Status::DEAD => msg.push_str("DEAD"),
                    Fb303Status::ALIVE => msg.push_str("ALIVE"),
                    Fb303Status::STARTING => msg.push_str("STARTING"),
                    Fb303Status::STOPPING => msg.push_str("STOPPING"),
                    Fb303Status::STOPPED => msg.push_str("STOPPED"),
                    Fb303Status::WARNING => msg.push_str("WARNING"),
                    _ => msg.push_str("UNKNOWN"),
                }
                let detail = client.get_status_details().unwrap();
                if detail.len() > 0 {
                    msg.push_str(" - ");
                    msg.push_str(&detail);
                }
                println!("{}", msg);
            }
            "counters" => {
                let res = client.get_counters().unwrap();
                for (k, v) in res {
                    println!("{}: {}", k, v);
                }
            }
            "name" => {
                let res = client.get_name().unwrap();
                println!("{}", res);
            }
            "version" => {
                let res = client.get_version().unwrap();
                println!("{}", res);
            }
            _ => {
                println!("unkown admin command");
            }
        },
    }
}
