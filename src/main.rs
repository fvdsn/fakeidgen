use rand::Rng;
use clap::{Arg, Command};

fn print_rand_id(rng: &mut rand::rngs::ThreadRng, json:bool) {
    let i = rng.gen_range(0..2);
    if i == 0 {
        if json {
            println!("{{\"name\":\"John Smith\"}}");
        } else {
            println!("name: John Smith");
        }
    } else {
        if json {
            println!("{{\"name\":\"Bob Doe\"}}");
        } else {
            println!("name: Bob Doe");
        }
    }
}
fn main() {
    // 1) Declare & Parse command line arguments
    let m = Command::new("fakeidgen")
        .version("0.1.0")
        .about("Generates plausible fake identities")
        .long_about("
This program generates a fake identity
        ")
        .author("Frédéric van der Essen")
        .arg(
            Arg::new("json")
                .long("json")
                .short('j')
                .takes_value(false)
                .help("Outputs the fake identity in the JSON format")
                .required(false)
        )
        .arg(
            Arg::new("num")
                .long("num")
                .short('n')
                .takes_value(true)
                .forbid_empty_values(true)
                .help("How many fake identities to generate (1-MAX_INT)")
                .required(false)
        )
        .after_help("")
        .get_matches();

    let num = if m.is_present("num") {
        m.value_of("num").unwrap().parse::<usize>().unwrap()
    } else { 1 };

    let json = m.is_present("json");

    let mut rng = rand::thread_rng();

    for _ in 0..num {
       print_rand_id(&mut rng, json);
    }
}
