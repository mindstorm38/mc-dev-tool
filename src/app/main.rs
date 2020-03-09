use mcdev::version::{self, MinecraftVersions, MinecraftVersion, MinecraftVersionType};
use mcdev::meta::{self, MinecraftMeta, MinecraftDownload};
use ansi_term::Colour::{Yellow, Red, Cyan};
use std::io::{stdin, stdout, Write};
use std::cmp::max;
use std::rc::Rc;

mod utils;
mod command;

use command::CommandManager;
use command::help::HelpCommand;

fn main() {

    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    println!();
    println!("{}", Yellow.bold().paint("Welcome to Minecraft Dev Tool !"));
    println!();

    let mut input = String::new();
    let mut running = true;

    let mut command_manager = CommandManager::new();

    command_manager.add_command(Box::new(HelpCommand::new()));

    let mut versions: Option<MinecraftVersions> = None;
    let mut version_meta: Option<MinecraftMeta> = None;

    while running {

        print!("{} ", Yellow.paint(">"));
        stdout().flush().unwrap();

        input.clear();

        if let Ok(_) = stdin().read_line(&mut input) {

            command_manager.execute(input.trim());

            /*
            let args = input.trim().split_whitespace().collect::<Vec<&str>>();

            if args.len() != 0 {

                match args[0] {

                    "help" => print_help(),
                    "exit" => running = false,

                    "vlist" => print_versions(&args, &mut versions),
                    "vsel" => select_version(&args, &mut versions, &mut version_meta),

                    "info" => print_info(&version_meta),
                    "libs" => print_libs(&version_meta),

                    _ => println!("{}", Red.paint("Invalid command !"))

                }

            }*/

        }

    }

}

fn print_help() {

    println!(" - {}", Cyan.paint("help"));
    println!(" - {}", Cyan.paint("exit"));
    println!(" - {}", Cyan.paint("vlist [<type>] [latest]"));
    println!(" - {}", Cyan.paint("vsel <version>"));
    println!(" - {}", Cyan.paint("info"));
    println!(" - {}", Cyan.paint("libs"));

}

fn get_versions(versions: &mut Option<MinecraftVersions>) -> &Option<MinecraftVersions> {

    if versions.is_none() {
        *versions = version::request_versions();
        if versions.is_none() {
            println!("{}", Red.paint("Failed to request versions."));
        }
    }

    versions

}

fn print_versions(args: &Vec<&str>, versions: &mut Option<MinecraftVersions>) {

    if let Some(versions) = get_versions(versions) {

        let mut filter_type: Option<MinecraftVersionType> = None;
        let mut filter_latest = false;

        for i in 1..=2 {
            if args.len() > i {
                if let Some(typ) = MinecraftVersionType::from_name(&String::from(args[i])) {
                    filter_type = Some(typ);
                } else if args[i] == "latest" {
                    filter_latest = true;
                } else {
                    println!("{} {} {}", Red.paint("Invalid keyword"), Yellow.paint(args[i]), Red.paint("!"))
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

    let id = &version.id;
    let padding = utils::fill_string(max(24 - id.len() as isize, 0), ' ');

    println!(" - {}{}[{}]", Cyan.paint(id), padding, Yellow.paint(version.typ.to_string()))

}

fn select_version(args: &Vec<&str>, versions: &mut Option<MinecraftVersions>, version_meta: &mut Option<MinecraftMeta>) {

    if args.len() < 2 {

        println!("Usage : {}", Cyan.paint("vsel <version>"));
        return;

    }

    let version_id = String::from(args[1]);

    if let Some(versions) = get_versions(versions) {

        if let Some(meta) = version_meta {
            if meta.id == version_id {
                return;
            }
        }

        if let Some(version) = versions.get_version(&version_id) {

            if let Some(meta) = meta::request_version_meta(version) {

                *version_meta = Some(meta);
                println!("{} {} {}", Cyan.paint("Version"), Yellow.paint(version_id), Cyan.paint("selected."))

            } else {
                println!("{}", Red.paint("Failed to request version metadata."));
            }

        } else {
            println!("{} {}{}", Red.paint("Invalid version"), Yellow.paint(version_id), Red.paint("."))
        }


    }

}

fn print_info(version_meta: &Option<MinecraftMeta>) {

    if let Some(meta) = version_meta {

        println!();
        println!(" {} {}", Cyan.paint("           Name :"), Yellow.paint(&meta.id));
        println!(" {} {}", Cyan.paint("           Type :"), Yellow.paint(meta.typ.to_string()));
        println!(" {} {}", Cyan.paint(" Assets version :"), Yellow.paint(&meta.assets));
        println!(" {} {}", Cyan.paint("     Main class :"), Yellow.paint(&meta.main_class));
        println!(" {} {}", Cyan.paint("     Libs count :"), Yellow.paint(meta.libraries.len().to_string()));

        println!(" {} {}", Cyan.paint("    Client .jar :"), format_download(&meta.client_downloads.jar));

        if let Some(mapping) = &meta.client_downloads.mappings {
            println!(" {} {}", Cyan.paint("Client mappings :"), format_download(mapping));
        }

        if let Some(server_downloads) = &meta.server_downloads {

            if let Some(mapping) = &server_downloads.mappings {
                println!(" {} {}", Cyan.paint("    Server .jar :"), format_download(mapping));
            }

            if let Some(mapping) = &server_downloads.mappings {
                println!(" {} {}", Cyan.paint("Server mappings :"), format_download(mapping));
            }

        }

        println!();

    } else {
        println!("{}", Red.paint("Please select version before showing information."));
    }

}

fn format_download(dl: &MinecraftDownload) -> String {
    format!("{} {}{}{}", Yellow.paint(&dl.url), Cyan.paint("("), Yellow.paint(format!("{}o", dl.size)), Cyan.paint(")"))
}

fn print_libs(version_meta: &Option<MinecraftMeta>) {

    if let Some(meta) = version_meta {

        println!(" {} {} {}{}{}",
                 Cyan.paint("Minecraft libraries for version"),
                 Yellow.paint(&meta.id),
                 Cyan.paint("("),
                 Yellow.paint(meta.libraries.len().to_string()),
                 Cyan.paint(")")
        );

        for lib in &meta.libraries {

            let padding = utils::fill_string(max(50 - lib.name.len() as isize, 0), ' ');

            println!(" - {}{}[{}]",
                     Cyan.paint(&lib.name),
                    padding,
                     Yellow.paint(if let Some(url) = &lib.url { url } else { "No URL" })
            );

        }

    } else {
        println!("{}", Red.paint("Please select version before showing libraries."));
    }

}