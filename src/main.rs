use std::process::Command;
use clap::App;
use std::env;

mod commands {
    pub(crate) mod install;
}

fn main() {
    let commands = vec![
        crate::commands::install::command_config(),
    ];

    let app = App::new("ring")
        .version("0.1.0")
        .author("Mlanawo Mbechezi <mlanawo.mbechezi@kemeter.io>")
        .about("Javascript package manager")
        .subcommands(commands);

    let matches = app.get_matches();
    let subcommand_name = matches.subcommand_name();

    match subcommand_name {
        Some("install") => {
            crate::commands::install::install(
                matches.subcommand_matches("install").unwrap(),
            );
        }
        _ => {
            let process_args: Vec<String> = env::args().collect();
            let process_name = process_args[0].as_str().to_owned();

            let mut subprocess = Command::new(process_name.as_str())
                .arg("--help")
                .spawn()
                .expect("failed to execute process");

            subprocess
                .wait()
                .expect("failed to wait for process");
        }
    }
}
