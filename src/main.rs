extern crate rand;

use rand::Rng;
use std::env;

const LETTER_BYTES: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE_BYTES: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBER_BYTES: &str = "0123456789";
const SPECIAL_CHARS: &str = "!@#$%^&*()";

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() <= 1 || args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
    print_help();
    return;
  }

  let mut lenght =8;
  let mut use_numbers = false;
  let mut use_uppercase = false;
  let mut use_special_chars = false;

  for i in 1..args.len() {
    match args[i].as_str() {
      "-l" | "--length" => {
        if let Some(len) = args.get(i + 1) {
          lenght = len.parse().unwrap_or(8);
        }
      }
      "-n" | "--numbers" => {
        use_numbers = true;
      }
      "-u" | "--uppercase" => {
        use_uppercase = true;
      }
      "-s" | "--special-chars" => {
        use_special_chars = true;
      }
      _ => {}
    }
  }

  let mut password_builder = String::new();

  if use_uppercase {
    password_builder.push_str(UPPERCASE_BYTES);
  }
  password_builder.push_str(LETTER_BYTES);
  if use_numbers {
    password_builder.push_str(NUMBER_BYTES);
  }
  if use_special_chars {
    password_builder.push_str(SPECIAL_CHARS);
  }

  let password = generate_random_password(&password_builder, lenght);
  println!("{}", password);
}

fn generate_random_password(charset: &str, lenght: usize) -> String {
  let mut rng = rand::thread_rng();
  let password: String = (0..lenght)
    .map(|_| {
      let idx = rng.gen_range(0..charset.len());
      charset.chars().nth(idx).unwrap()
    })
    .collect();
  password
}

fn print_help() {
  println!("Password Generator CLI Tool");
    println!("Usage: password-generator [options]");
    println!("Options:");
    println!("  -l <length>, --length <length>       Desired password length (default: 8)");
    println!("  -n, --numbers                        Include numbers in the password");
    println!("  -u, --uppercase                      Include uppercase letters in the password");
    println!("  -s, --specialchars                   Include special characters in the password");
    println!("  --help                               Show help message");
}


