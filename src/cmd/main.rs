use std::io;

use argh::FromArgs;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel};

use scribec::fb303::{Fb303Status, TBaseServiceSyncClient};
use scribec::scribe::{LogEntry, ScribeSyncClient, TScribeSyncClient};

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

    let mut c = TTcpChannel::new();
    c.open(format!("{}:{}", args.host, args.port)).unwrap();
    let (i_chan, o_chan) = c.split().unwrap();

    let i_prot = TBinaryInputProtocol::new(TFramedReadTransport::new(i_chan), false);
    let o_prot = TBinaryOutputProtocol::new(TFramedWriteTransport::new(o_chan), false);

    let mut client = ScribeSyncClient::new(i_prot, o_prot);

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
            "alive" | "status" => {
                let res = client.get_status().unwrap();
                match res {
                    Fb303Status::DEAD => println!("DEAD"),
                    Fb303Status::STARTING => println!("STARTING"),
                    Fb303Status::ALIVE => println!("ALIVE"),
                    Fb303Status::STOPPING => println!("STOPPING"),
                    Fb303Status::STOPPED => println!("STOPPED"),
                    Fb303Status::WARNING => println!("WARNING"),
                    _ => println!("UNKNOWN"),
                }
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
