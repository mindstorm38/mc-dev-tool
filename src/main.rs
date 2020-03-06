use mcdev::mcapi;
use std::io::{stdin, stdout, Write};
use ansi_term::Colour::{Yellow, Red, Cyan};
use mcdev::mcapi::{MinecraftVersions, MinecraftVersion, MinecraftVersionType};

fn main() {

    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    println!();
    println!("  {0} {1} {0} ", Yellow.paint("####"), Yellow.bold().paint("Minecraft Dev Tool"));
    println!();

    let mut input = String::new();
    let mut running = true;

    let mut versions: Option<MinecraftVersions> = None;

    while running {

        print!("{} ", Yellow.paint(">"));
        stdout().flush().unwrap();

        input.clear();

        if let Ok(_) = stdin().read_line(&mut input) {

            let args = input.trim().split_whitespace().collect::<Vec<&str>>();

            if args.len() == 0 {
                println!("{}", Red.paint("Not a command !"));
            } else {
                match args[0] {
                    "help" => print_help(),
                    "versions" => print_versions(&args, &mut versions),
                    _ => println!("{}", Red.paint("Invalid command !"))
                }
            }

        }

    }

    //let versions = mcapi::request_versions();

    //dbg!(versions);

}

fn print_help() {

    println!("{}", Cyan.paint("versions [<type>] [latest]"));
    println!("{}", Cyan.paint("version <version>"));

}

fn print_versions(args: &Vec<&str>, versions: &mut Option<MinecraftVersions>) {

    if versions.is_none() {
        *versions = mcapi::request_versions();
    }

    if let Some(versions) = versions {

        let mut filter_type: Option<MinecraftVersionType> = None;
        let mut filter_latest = false;

        for i in 1..2 {
            if args.len() > i {
                if let Some(typ) = MinecraftVersionType::from_name(&String::from(args[i])) {
                    filter_type = Some(typ);
                } else if args[i] == "latest" {
                    filter_latest = true;
                } else {
                    println!("{} {} {}", Red.paint("Invalid keyword"), Yellow.paint(args[i]), Red.paint(" !"))
                }
            }
        }

        for vid in versions.get_versions() {
            if let Some(version) = versions.get_version(vid) {

                if let Some(typ) = &filter_type {
                    if version.typ != *typ {
                        continue;
                    }
                }

                if filter_latest && !versions.is_latest(&version) {
                    continue;
                }

                print_version(&version);

            }
        }

    }

}

fn print_version(version: &MinecraftVersion) {
    println!("{} [{}]", Cyan.paint(*version.id), Yellow.paint(*version.typ))
}