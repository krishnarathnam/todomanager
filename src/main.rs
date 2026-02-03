fn main() {
    todomanager::init();
    let args = todomanager::get_args();

    match args.command.as_str() {
        "add" => todomanager::add(args.arguments),
        "list" => todomanager::list(),
        "toggle" => todomanager::toggle(args.arguments),
        "rm" => todomanager::remove(args.arguments),
        "help" => todomanager::help(),
        "reset" => todomanager::reset(),
        "sort" => todomanager::sort(),
        _ => {
            todomanager::help();
        }
    }
}
