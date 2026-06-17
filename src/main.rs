use std::env::args;
use color_eyre::Result;
use std::process;
mod app;
mod ui;
mod editor;

fn main() -> Result<()> {
    color_eyre::install()?;
    let args:Vec<String> = args().collect();
    let mut app = app::App::default();
    if args.len() > 1 {
        exec_args(&args, &mut app);
    }
    ratatui::run(|terminal| app.run(terminal))
}

fn exec_args(args: &Vec<String>, app: &mut app::App){
    for arg in args.iter().skip(1) {
        if arg == "help" {
            help_info();
            process::exit(0);
        } else if arg.ends_with(".json") { //If file is json
            //If json can't be parsed
            if let Err(e) = import_json_file(arg, app) {
                println!("{:?}", e);
                process::exit(1);
            }
        } else { //If arg is not valid
            println!("ERROR: Invalid argument: {arg}");
            process::exit(1);
        }
    }
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
    CTRL s - Save list
    CTRL e - Enter editing mode
    CTRL v - Enter viewing mode
    ENTER - Toggle item as complete or todo

    (While in editing mode)
    CTRL c - Stop editing text
    i - insert text
    TAB - change windows

    Lists are stored in json form.
    To load a list on start up, add the file to args");

}

fn import_json_file(file_name: &str, app: &mut app::App) -> Result<()> {
    app.list.read_list_from_file(file_name)
}
