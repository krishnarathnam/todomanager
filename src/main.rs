fn main() {
    todomanager::init();
    let args = todomanager::get_args();

    match args.command.as_str() {
        "add" => todomanager::add(args.arguments),
        "list" => todomanager::list(),
        "done" => todomanager::done(args.arguments.get(0).unwrap_or(&String::new()).clone()),
        "undo" => todomanager::undo(args.arguments.get(0).unwrap_or(&String::new()).clone()),
        "rm" => todomanager::remove(args.arguments.get(0).unwrap_or(&String::new()).clone()),
        "help" => todomanager::help(),
        "reset" => todomanager::reset(),
        "sort" => todomanager::sort(),
        _ => {
            todomanager::help();
        }
    }
}
