use clap::{App, Arg};
use km::*;
fn main() {
    let app_m = App::new("Knowledge management Program")
        .version(crate_version!())
        .author("brian")
        .about("Knowledge Manager")
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .takes_value(false)
                .help("List all the tags"),
        )
        .subcommand(
            App::new("init")
                .about(
                    "init storage db",
                )
                .version(crate_version!())
        )
        .subcommand(
            App::new("open")
                .about(
                    "open mark by tag",
                )
                .version(crate_version!())
                .arg(
                    Arg::new("TAG")
                        .required(true)
                        .index(1)
                        .help("Open mark for this tag"),
                ),
        )
        .subcommand(
            App::new("add")
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
            App::new("del")
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
            App::new("update")
                .about("update tag")
                .version(km::crate_version!())
                .arg(
                    Arg::new("OLDTAG")
                        .required(true)
                        .index(1)
                        .help("set old tag")
                )
                .arg(
                    Arg::new("NEWTAG")
                        .required(true)
                        .index(2)
                        .help("Set new tag"),
                ),
        )
        .get_matches();
    if app_m.is_present("list") {
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
            if let Some(tag) = sub_m.value_of("TAG") {
                match read_mark(tag) {
                    Err(why) => println!("Read mark for {} failed. {}", tag, why),
                    Ok(mark) => match open::that(&mark) {
                        Ok(()) => println!("Open {} success.", &mark),
                        Err(err) => eprintln!("Error occurred when open {}: {}", &mark, err)
                    }
                }
            }
        }
        _ => {}
    }
}
