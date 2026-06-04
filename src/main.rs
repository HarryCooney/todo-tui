use std::env::args;
use color_eyre::Result;
mod app;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args:Vec<String> = args().collect();
    if args.len() > 1 {
        return exec_args(&args);
    }
    let test = app::App::default();
    test.list.serialize()?;
    ratatui::run(|terminal| test.run(terminal))
}

fn exec_args(args: &Vec<String>) -> Result<()> {
    for arg in args.iter().skip(1) {
        if arg == "help" {
            help_info();
        } 
        else {
            panic!("Invalid argument: {arg}");
        }
    }
    Ok(())
}

fn help_info() {
    println!("
    Hello! Welcome to my Tuido app (The i is silent)
    These are the controls, I hope they work for you!\n
    q - Quit app
    k - Select previous task in list
    j - Select next task list
    x - Delete task
    g - Select first task in list
    G - Select last task in list
    h - Deselect current task
    ENTER - Toggle item as complete or todo");
}
