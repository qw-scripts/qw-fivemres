use clap::Parser;

#[derive(Parser, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    project_name: String,
}

fn create_dirs(project_name: &str) {

    std::fs::create_dir(String::from(project_name)).unwrap();

    std::fs::create_dir(format!("{}/client", String::from(project_name))).unwrap();

    std::fs::create_dir(format!("{}/server", String::from(project_name))).unwrap();

    std::fs::create_dir(format!("{}/shared", String::from(project_name))).unwrap();
}

fn write_files(project_name: &str) {
    std::fs::write(format!("{}/client/main.lua", String::from(project_name)), "local QBCore = exports['qb-core']:GetCoreObject()").unwrap();


    std::fs::write(format!("{}/server/main.lua", String::from(project_name)), "local QBCore = exports['qb-core']:GetCoreObject()").unwrap();


    std::fs::write(format!("{}/shared/config.lua", String::from(project_name)), "Config = {}\n \n-- DEBUG -- \nConfig.Debug = true").unwrap();

    std::fs::write(format!("{}/fxmanifest.lua", String::from(project_name)), "fx_version 'cerulean'\ngame 'gta5'\n\ndescription ''\nauthor ''\nversion '0.0.0'\n\nclient_scripts { \n 'client/**/*' \n }\n\nserver_scripts { \n 'server/**/*' \n }\n\nshared_scripts { \n 'shared/**/*' \n }\n\nlua54 'yes'").unwrap();

}

fn main() {
    let args = Cli::parse();

    let project_name = args.project_name;

    let now = std::time::Instant::now();

    create_dirs(&project_name);

    write_files(&project_name);

    println!("Created project in {}ms", now.elapsed().as_millis());
    println!("Have fun developing!");

}
