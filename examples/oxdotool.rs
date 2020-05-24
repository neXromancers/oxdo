use std::time::Duration;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        print_usage(&args[0]);
        return;
    }

    match args[1].as_str() {
        "help" => print_help(&args[0]),

        "type" => type_command(&args),

        _ => {
            println!("Command {} not found.\n", args[1]);
            print_help(&args[0]);
        }
    }
}

fn type_command(args: &[String]) {
    let to_type = &args[2..];

    if to_type.is_empty() {
        println!("Try typing something after \"type\"");
        return;
    }

    let full_text = to_type.join(" ");

    let ox = oxdo::OxDo::new(None);
    ox.enter_text_window(0, &full_text, Duration::from_millis(0));
}

fn print_usage(arg0: &str) {
    eprintln!("Usage: {} COMMAND ARGS", arg0);
    eprintln!("See {} help for a list of commands", arg0);
}

fn print_help(arg0: &str) {
    println!("Usage: {} COMMAND ARGS", arg0);
    println!("Available commands:");
    println!(" - type <text to type>");
}