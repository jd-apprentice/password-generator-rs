use clap::Parser;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 12)]
    length: u32,
}

fn main() {
    let args = Args::parse();

    let options = &[
        "Include Numbers",
        "Include Lowercase",
        "Include Uppercase",
        "Include Symbols",
    ];
    let defaults = &[true, true, true, false];

    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select character types to include")
        .items(&options[..])
        .defaults(&defaults[..])
        .interact()
        .unwrap();

    let use_numbers = selections.contains(&0);
    let use_lowercase = selections.contains(&1);
    let use_uppercase = selections.contains(&2);
    let use_symbols = selections.contains(&3);

    let password = password_generator::generate_password(
        args.length,
        use_numbers,
        use_lowercase,
        use_uppercase,
        use_symbols,
    );
    println!("Generated password: {}", password);
}
