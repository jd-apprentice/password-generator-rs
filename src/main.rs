use arboard::Clipboard;
use console::{style, Key, Term};
use password_generator::generate_password;
use std::{thread, time::Duration};

fn main() {
    let mut length: u32 = 12;
    let term = Term::stdout();
    let mut clipboard = Clipboard::new().unwrap();
    let options = &mut [
        ("Include Numbers", true),
        ("Include Lowercase", true),
        ("Include Uppercase", true),
        ("Include Symbols", false),
    ];
    let mut current_selection = 0;

    loop {
        term.clear_screen().unwrap();
        term.write_line("  _____                                    _    _____                           _             ")
            .unwrap();
        term.write_line(" |  __ \\                                  | |  / ____|                         | |            ")
            .unwrap();
        term.write_line(" | |__) |_ _ ___ _____      _____  _ __ __| | | |  __  ___ _ __   ___ _ __ __ _| |_ ___  _ __ ")
            .unwrap();
        term.write_line(" |  ___/ _` / __/ __\\ \\ /\\ / / _ \\| '__/ _` | | | |_ |/ _ \\ '_ \\ / _ \\ '__/ _` | __/ _ \\| '__|")
            .unwrap();
        term.write_line(" | |  | (_| \\__ \\__ \\\\ V  V / (_) | | | (_| | | |__| |  __/ | | |  __/ | | (_| | || (_) | |   ")
            .unwrap();
        term.write_line(" |_|   \\__,_|___/___/ \\_/\\_/ \\___/|_|  \\__,_|  \\_____|\\___|_| |_|\\___|_|  \\__,_|\\__\\___/|_|")
            .unwrap();
        term.write_line("").unwrap();
        term.write_line("Use up/down arrow keys to navigate.")
            .unwrap();
        term.write_line("Use space to toggle options.").unwrap();
        term.write_line("Use left/right arrow keys to adjust password length.")
            .unwrap();
        term.write_line("Press Enter to generate the password.")
            .unwrap();
        term.write_line("Press Esc or q to quit.").unwrap();
        term.write_line("").unwrap();
        term.write_line(&format!("Password Length: {}", style(length).green()))
            .unwrap();
        term.write_line("").unwrap();

        for (i, (name, selected)) in options.iter().enumerate() {
            let cursor = if i == current_selection {
                style(">").cyan()
            } else {
                style(" ")
            };
            let checkbox = if *selected {
                style("[x]").green()
            } else {
                style("[ ]").red()
            };
            term.write_line(&format!("{cursor} {checkbox} {name}"))
                .unwrap();
        }

        match term.read_key().unwrap() {
            Key::ArrowUp => current_selection = current_selection.saturating_sub(1),
            Key::ArrowDown => current_selection = (current_selection + 1).min(options.len() - 1),
            Key::ArrowLeft => length = (length).saturating_sub(1).max(8),
            Key::ArrowRight => length = (length).saturating_add(1).min(40),
            Key::Char(' ') => {
                if let Some(option) = options.get_mut(current_selection) {
                    option.1 = !option.1;
                }
            }
            Key::Enter => {
                let use_numbers = options[0].1;
                let use_lowercase = options[1].1;
                let use_uppercase = options[2].1;
                let use_symbols = options[3].1;

                let password = generate_password(
                    length,
                    use_numbers,
                    use_lowercase,
                    use_uppercase,
                    use_symbols,
                );
                term.write_line(&format!(
                    "Generated password: {}",
                    style(password.clone()).yellow()
                ))
                .unwrap();

                clipboard.set().text(password.to_owned()).unwrap();
                term.write_line(&style("Password copied to clipboard!").green().to_string())
                    .unwrap();
                thread::sleep(Duration::from_millis(100));
                break;
            }
            Key::Escape | Key::Char('q') => break,
            _ => {}
        }
    }
    let _ = clipboard;
}
