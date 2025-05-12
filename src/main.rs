use clap::Parser;
use coolbirthday::{app::App, cli::{Cli, Commands}, file::init};


fn main() {
    let cli = Cli::parse();

    if let Err(e) = init() {
        println!("Coudn't create main directory for CoolBirthday:\n{e}");
        return;
    }

    let mut app = match App::new(){
        Ok(b) => b,
        Err(e) => {
            println!("Error: Coudn't load birthday saves:\n{e}");
            App::default()
        },
    };

    match cli.command {
        Commands::List => app.list(),
        Commands::Add { name, month, day } => { 
            println!("{}", match app.add(name.clone(), month, day) {
                Ok(_) => format!("Added birthday of {name}: {day}/{month}"),
                Err(e) => format!("Error: {e}"),
            });
        },
        Commands::Nuke => {
            match App::nuke() {
                Ok(_) => println!("Nuking Cool Birthday"),
                Err(e) => println!("Error, couln't nuke: {e}"),
            };
        },
    };

}
