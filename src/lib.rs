use rand::Rng;

const NUMBERS: &[u8] = b"0123456789";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SYMBOLS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";

pub fn generate_password(
    length: u32,
    use_numbers: bool,
    use_lowercase: bool,
    use_uppercase: bool,
    use_symbols: bool,
) -> String {
    let mut charset = Vec::new();
    if use_numbers {
        charset.extend_from_slice(NUMBERS);
    }
    if use_lowercase {
        charset.extend_from_slice(LOWERCASE);
    }
    if use_uppercase {
        charset.extend_from_slice(UPPERCASE);
    }
    if use_symbols {
        charset.extend_from_slice(SYMBOLS);
    }

    if charset.is_empty() {
        return String::from("Please select at least one character type.");
    }

    let mut rng = rand::rng();
    (0..length)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect()
}
