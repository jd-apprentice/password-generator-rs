pub fn password_strength(password: &str) -> &str {
    let mut score = 0;

    if password.len() >= 8 {
        score += 1;
    }
    if password.len() >= 12 {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_digit()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_lowercase()) {
        score += 1;
    }
    if password.chars().any(|c| c.is_ascii_uppercase()) {
        score += 1;
    }
    if password
        .chars()
        .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c))
    {
        score += 1;
    }

    match score {
        0..=2 => "Weak",
        3..=4 => "Medium",
        5..=6 => "Strong",
        _ => "Very Strong",
    }
}
