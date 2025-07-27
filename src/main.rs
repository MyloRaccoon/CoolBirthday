use clap::Parser;
use coolbirthday::{app::App, cli::{Cli, Commands}, file::init, popup::popup};
use std::process::ExitCode;


/*
EXIT CODES:
0: success
1: save error
2: add error
*/
fn main() -> ExitCode {

    let cli = Cli::parse();

    let mut exit_code = ExitCode::SUCCESS;

    if let Err(e) = init() {
        println!("Coudn't create main directory for CoolBirthday:\n{e}");
    }

    let mut app = match App::new(){
        Ok(b) => b,
        Err(e) => {
            println!("Error: Coudn't load birthday saves:\n{e}");
            App::default()
        },
    };

    match cli.command {
        Some(command) => {
            match command {
                Commands::List => app.list(),


                Commands::Check { name } => {
                    match name {
                        Some(n) => {
                            match app.check(n.clone()) {
                                Some(b) => {
                                    if b {
                                        println!("Today is {n}'s birthday !");
                                    } else {
                                        println!("{}", app.get(n).unwrap());
                                    }
                                },
                                None => println!("Birthday of {n} doesn't exists"),
                            }
                        },

                        None => match app.check_all() {
                            Some(birthday) => println!("Today is {}'s birthday !", birthday.name),
                            None => {
                                match app.get_next() {
                                    Some(birthday) => println!("Next birthday is {birthday}"),
                                    None => println!("Please add some birthday before (coolbirthday add)"),
                                };
                            },
                        },
                    }
                },


                Commands::Add { name, month, day } => { 
                    if app.birthday_exists(name.clone()) {
                        println!("Birthday of {name} already exists");
                    } else {
                        match app.add(name.clone(), month, day) {
                            Ok(birthday) => println!("Added birthday {birthday}"),
                            Err(e) => {println!("Error: {e}"); exit_code = ExitCode::from(2)},
                        };
                    }
                },


                Commands::Remove { name } => {
                    if app.birthday_exists(name.clone()) {
                        app.remove(name.clone());
                        println!("Removed birthday of {name}");
                    } else {
                        println!("Birthday of {name} doesn't exists");
                    }
                },


                Commands::Nuke => {
                    println!("Nuking Cool Birthday");
                    app.nuke();
                }
            }
        },
        None => {
            match app.check_all() {
                Some(birthday) => popup(format!("Today is {}'s birthday !", birthday.name)),
                None => {
                    println!("No birthday today :(");
                    println!("type 'coolbirthday help' to see commands");
                },
            }
        }
    };

    match app.save() {
        Ok(c) => c,
        Err(e) => {
            println!("Error, couln't save: {e}");
            exit_code = ExitCode::from(1);
        },
    };

    exit_code
}