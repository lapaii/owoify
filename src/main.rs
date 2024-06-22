use std::env;

fn main() {
    let dict = vec![
        // define the dictionary of values to find and replace
        ("r", "w"),
        ("l", "w"),
        ("you", "uwu"),
        ("no", "nu"),
        ("has", "haz"),
        ("have", "haz"),
        ("to", "tu"),
        ("too", "twoo"),
        ("love", "wuv"),
        ("cute", "kawaii"),
        ("kiss", "chu"),
        ("good", "gud"),
        ("that", "dat"),
        ("this", "dis"),
        ("thing", "thingy"),
        ("with", "wif"),
        ("for", "fuwa"),
        ("thank", "fank"),
        ("please", "pwease"),
        ("sorry", "sowwy"),
    ];

    /*
        get the args:

        can be:
        0 -> location of the binary
        1 -> text to be owo-fied

        or:
        0 -> location of the binary
        1 onwards -> text to be owo-fied
    */
    let mut args: Vec<String> = env::args().collect();
    let version = env!("CARGO_PKG_VERSION");

    args.remove(0); // removes the location of the binary from args

    if args.is_empty() || args[0].to_lowercase() == "--h" || args[0].to_lowercase() == "--help" {
        print!(
            "usage: owoify <input text>\n\noptions:\n\n--h --help print help\n--v --version print version\n\nversion: {}\n",
            version
        );
        std::process::exit(0);
    }

    if args[0].to_lowercase() == "--version" || args[0].to_lowercase() == "--v" {
        println!("version: {}", version);
        std::process::exit(0);
    }

    let mut input = args.join(" ").to_lowercase(); // joins the remaining args into one string and forces lowercase

    for (pattern, replacement) in &dict {
        input = input.replace(pattern, replacement); // actually replace the string
    }

    println!("{} ", input); // output and end program
}
