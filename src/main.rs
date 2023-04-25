use arcdps_log_tools::{extract_casts, extract_positions};
use clap::{error::ErrorKind, CommandFactory, Parser};

mod args;

use self::args::*;

fn main() {
    let Cli { command, args } = Cli::parse();

    if args.input.is_empty() {
        Cli::command()
            .error(
                ErrorKind::MissingRequiredArgument,
                "input file was not provided",
            )
            .exit();
    }

    let log = args.open_log();
    let events = args.filter_log(&log);

    match command {
        Command::All => {
            let events: Vec<_> = events.map(|event| (event.kind(), event)).collect();
            println!("Found {} events", events.len());
            args.write_output(&events);
        }

        Command::Cast { skill: skill_arg } => {
            let skill = log
                .skills
                .iter()
                .find(|skill| match skill_arg.parse::<u32>() {
                    Ok(id) => skill.id == id,
                    Err(_) => skill.name == skill_arg,
                })
                .unwrap_or_else(|| panic!("Skill \"{}\" not found", skill_arg));
            println!("Finding casts of skill \"{}\" ({})", skill.name, skill.id,);

            let data = extract_casts(&log, events, skill.id);
            println!(
                "Found {} casts and {} hits without cast",
                data.casts.len(),
                data.hits_without_cast.len()
            );

            args.write_output(&data);
        }

        Command::Position => {
            println!("Finding positions");

            let positions = extract_positions(&log, events);
            println!("Found {} positions", positions.len());
            if let Some(pos) = positions.first() {
                println!("Initial position at {} {} {}", pos.x, pos.y, pos.z);
            }

            args.write_output(&positions);
        }

        Command::BuffInfo => todo!("buff info extraction"),
    }
}
