//number generator 
fn main() {
    let mut rng = rand::thread_rng();
    let mut number = rng.gen_range(1, 26);
    println!("Your number is {}", number);

    let mut input = String::new();
    println!("Enter a string to encrypt: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut input = input.trim().to_string();
    let mut output = String::new();
    for c in input.chars() {
        let mut c = c as u8;
        if c >= 65 && c <= 90 {
            c = c + number;
            if c > 90 {
                c = c - 26;
            }
        } else if c >= 97 && c <= 122 {
            c = c + number;
            if c > 122 {
                c = c - 26;
            }
        }
        output.push(c as char);
    }
    println!("Your encrypted string is {}", output);
}


//encryption function
fn encrypt(text: &str, shift: i32) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            result.push((((c as u8 - offset + shift as u8) % 26) + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

//decryption function
fn decrypt(text: &str, shift: i32) -> String {
    let mut result = String::new();
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let offset = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            result.push((((c as u8 - offset - shift as u8) % 26) + offset) as char);
        } else {
            result.push(c);
        }
    }
    result
}

