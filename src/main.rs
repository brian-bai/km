use clap::{Arg, ArgMatches, Command};
use km::*;
fn main() {
    let app_m: ArgMatches = Command::new("Knowledge management Program")
        .version(crate_version!())
        .author("brian")
        .about("Knowledge Manager")
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .num_args(0)
                .help("List all the tags"),
        )
        .subcommand(
            Command::new("init")
                .about("init storage db")
                .version(crate_version!()),
        )
        .subcommand(
            Command::new("open")
                .about("open mark by tag")
                .version(crate_version!())
                .arg(
                    Arg::new("TAG")
                        .required(true)
                        .index(1)
                        .help("Open mark for this tag"),
                ),
        )
        .subcommand(
            Command::new("add")
                .about("add new mark")
                .version(km::crate_version!())
                .arg(Arg::new("TAG").required(true).index(1).help("Set the tag"))
                .arg(
                    Arg::new("MARK")
                        .required(true)
                        .index(2)
                        .help("Set the mark"),
                ),
        )
        .subcommand(
            Command::new("del")
                .about("delete a mark")
                .version(km::crate_version!())
                .arg(
                    Arg::new("TAG")
                        .required(true)
                        .index(1)
                        .help("delete the mark for this tag"),
                ),
        )
        .subcommand(
            Command::new("update")
                .about("update tag")
                .version(km::crate_version!())
                .arg(
                    Arg::new("OLDTAG")
                        .required(true)
                        .index(1)
                        .help("set old tag"),
                )
                .arg(
                    Arg::new("NEWTAG")
                        .required(true)
                        .index(2)
                        .help("Set new tag"),
                ),
        )
        .get_matches();

    if app_m.contains_id("list") {
        println!("List all tags:");
        match read_tags() {
            Err(why) => print!("Read tags failed. {:?}", why),
            Ok(tags) => {
                for tag in tags {
                    println!("........ {}", tag);
                }
            }
        }
    }

    match app_m.subcommand() {
        Some(("init", _sub_m)) => {
            println!("Init the storage DB: ");
            init_storage().expect("Init storage");
        }
        Some(("open", sub_m)) => {
            let tag: &str = *sub_m.get_one("TAG").expect("Tag is required");

            match read_mark(tag) {
                Err(why) => println!("Read mark for {} failed. {}", tag, why),
                Ok(mark) => match open::that(&mark) {
                    Ok(()) => println!("Open {} success.", &mark),
                    Err(err) => eprintln!("Error occurred when open {}: {}", &mark, err),
                },
            }
        }
        Some(("add", sub_m)) => {
            let tag: &str = *sub_m.get_one("TAG").expect("Tag is required");

            let mark: &str = *sub_m.get_one("MARK").expect("Mark is required");
            match add_mark(&tag, &mark) {
                Ok(()) => println!("Add {}:{} success.", tag, mark),
                Err(why) => println!("Add mark for {} failed. {}", tag, why),
            }
        }
        Some(("del", sub_m)) => {
            let tag: &str = *sub_m.get_one("TAG").expect("Tag is required");
            match del_mark(tag) {
                Ok(()) => println!("{} deleted.", tag),
                Err(why) => println!("Delete mark for {} failed. {}", tag, why),
            }
        }
        Some(("update", sub_m)) => {
            let tag: &str = *sub_m.get_one("OLDTAG").expect("Old Tag is required");
            let newtag: &str = *sub_m.get_one("NEWTAG").expect("New Tag is required");

            match update_mark(tag, newtag) {
                Ok(()) => println!("Update tag {}:{} success.", tag, newtag),
                Err(why) => println!("Update tag for {} failed. {}", tag, why),
            }
        }

        _ => {}
    }
}
