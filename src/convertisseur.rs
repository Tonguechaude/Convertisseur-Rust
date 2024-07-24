use std::fmt::Debug;
use std::{io, u64};
use std::io::{Read};

const ENTER : i8 =  13;
const TAB : i8 = 9;
const BKSP : i8 = 8;

fn main()
{
    welcome_screen();
}

fn welcome_screen ()
{
    loop
    {
        screen_cleaner();
        println!("-------------------------------------------");
        println!(">>> Welcome to Number System Converter <<<");
        println!("------------------------------------------- \n");

        println!(">> Select Conversion Type:");
        println!("> 1. Binary Conversion");
        println!("> 2. Decimal Conversion");
        println!("> 3. Octal Conversion");
        println!("> 4. Hexadecimal Conversion");
        println!("> 5. Exit the Program \n \n");

        println!("Enter the number & Hit ENTER: ");

        let mut input = String::new(); //crée un String
        io::stdin()
            .read_line(&mut input) // lit ce qui est écrit dans l’entrée standard et le met dans input
            .expect("Failed to read line");

        let choice: i32 = match input
            .trim()  // pour supprimer les espaces blancs y compris les \n
            .parse()  // pour transformer la string en int
        {
            Ok(num) => num,
            Err(_) => {
                println!("Error: the number must be between 1 to 5.");
                println!("Press any key to continue...");
                wait_for_keypress();
                continue;
            }
        };

        match choice {
            1 => user_input(1),
            2 => user_input(2),
            3 => user_input(3),
            4 => user_input(4),
            5 => exit_screen(),
            _ => {
                println!("\nError: the number must be between 1 to 5.\n");
                println!("Press any key to continue...");
                wait_for_keypress();
                welcome_screen();
            }
        }
    }

}

fn user_input (choice : i32)
{
    screen_cleaner();

    match choice // code pour le binaire
    {
        1 => {
            println!("Enter the binary: ");
            let bi = read_input::<u64>();

            if !digit_checker(bi, choice)
            {
                println!("\nError: Binary can only have the digit 0, 1.");
                println!("Press any key to continue...");
                wait_for_keypress();
                welcome_screen();
            }
            else
            {
                conversion_title();
                println!("Octal number : {}", binary_to_octal(bi));
                println!("Decimal number : {}", binary_to_decimal(bi));
                println!("hexadecimal number : {}", binary_to_hexadecimal(bi));
                try_again(choice);
            }
        },

        2 => {  // Code pour le Decimal
            println!("Enter the decimal: ");
            let deci = read_input::<u64>();

            if deci <= 0 || !digit_checker(deci, choice)
            {
                println!("\nError: Decimal number can't be negative.");
                println!("Press any key to continue...");
                wait_for_keypress();
                welcome_screen();
            }
            else
            {
                conversion_title();
                println!("Binary number : {}", decimal_to_binary(deci));
                println!("Octal number : {}", decimal_to_octal(deci));
                println!("hexadecimal number : {}", decimal_to_hexadecimal(deci));
                try_again(choice);
            }
        },

        3 => {  // Code pour l’Octal
            println!("Enter the octal: ");
            let octal = read_input::<u64>();

            if !digit_checker(octal, choice)
            {
                println!("\nError: Octal digits can only be between 0 to 7.");
                println!("Press any key to continue...");
                wait_for_keypress();
                welcome_screen();
            }
            else
            {
                conversion_title();
                println!("Binary number : {}", octal_to_binary(octal));
                println!("Decimal number : {}", octal_to_decimal(octal));
                println!("Hexadecimal number : {}", octal_to_hexadecimal(octal));

                try_again(choice);
            }
        },

        4 => {  // Code pour l'Hexadecimal
            println!("Enter the hexadecimal: ");
            let hexa : String = read_input::<String>();

            if hexa.chars().all(|c| c.is_digit(16))
            {
                println!("\n");
                conversion_title();
                println!("Binary number : {}", hexadecimal_to_binary(&hexa));
                println!("Octal number : {}", hexadecimal_to_octal(&hexa));
                println!("Decimal number : {}", hexadecimal_to_decimal(&hexa));
                try_again(choice);
            }
            else
            {
                println!("\n\nError: Hexadecimal digits can only be between 0 to 9 & A to F.");
                println!("Press any key to continue...");
                wait_for_keypress();
                welcome_screen();
            }
        },

        // Normalement ça n’arrive jamais
        _ => println!("\n>> Unexpected Error occurred. Report to program Administrator <<")
    }
}

fn digit_checker(num: u64, choice: i32) -> bool
{
    let mut temp = num;

    while temp != 0 {
        let rem = temp % 10;

        let valid = match choice {
            1 => rem == 0 || rem == 1,  // binary
            2 => rem >= 0 && rem <= 9,  // decimal
            3 => rem >= 0 && rem <= 7,  // octal
            _ => false,
        };

        if !valid {
            return false;
        }

        temp /= 10;
    }

    true
}

fn read_input<T: std::str::FromStr>() -> T
where
    <T as std::str::FromStr>::Err: Debug,
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Invalid input")
}

fn screen_cleaner ()
{
    use std::process::Command;
    Command::new("clear")
        .status()
        .unwrap();
}

fn reset_color ()
{
    println!("\x1B[0m");

}

fn wait_for_keypress()
{
    let _ = io::stdin() // accede à l’entrée standard
        .read(&mut [0u8]).unwrap(); // bloque le programme en attendant que l’utilisateur appuie sur une touche
}

fn conversion_title()
{
    println!("\n---------------------------");
    println!(">>> Conversion Results <<<");
    println!("---------------------------");
}

fn try_again(choice: i32)
{
    println!("\n\nDo you want to try again [y/N]: ");

    // Lire l’entrée de l’utilisateur
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input
        .trim();

    match input {
        "Y" | "y" => user_input(choice),
        "N" | "n" => welcome_screen(),
        _ => {
            println!("\nError: invalid input. \n");
            println!("Press any key to continue... \n");
            wait_for_keypress();
            welcome_screen();
        }
    }
}

fn exit_screen()
{
    screen_cleaner();
    println!("-------------------------------------------");
    println!(" >>> Creator : Tonguechaude <<< ");
    println!("-------------------------------------------");

    println!("> GitHub: https://github.com/Tonguechaude");
    println!("> LinkedIn: https://www.linkedin.com/in/\n");

    std::process::exit(0);
}

/* Conversions */

// fonctions conversions binaire → décimal
pub fn binary_to_octal (binary: u64) -> u64
{
    let mut num = binary;
    let mut decimal = 0;
    let mut i = 0;

    // Convertir le nombre binaire en décimal
    while num != 0 {
        let rem = num % 10;
        num /= 10;
        decimal += rem * 2_u64.pow(i);
        i += 1;
    }

    let mut octal = Vec::new();

    // Convertir le nombre décimal en octal
    let mut temp = decimal;
    while temp != 0 {
        octal.push((temp % 8) as u64);
        temp /= 8;
    }

    // Convertir le vecteur en nombre octal
    let mut octal_number = 0;
    for (i, &digit) in octal.iter().enumerate() {
        octal_number += digit * 10_u64.pow(i as u32);
    }

    return octal_number;
}

// fonctions conversions binaire → décimal
pub fn binary_to_decimal (binary: u64) -> u64
{
    let mut num = binary;
    let mut decimal = 0;
    let mut i = 0;

    while num != 0 {
        let rem = num % 10;
        num /= 10;
        decimal += rem * 2_u64.pow(i);
        i += 1;
    }

    return decimal;
}

// fonctions conversions binaire → hexadécimal
pub fn binary_to_hexadecimal (binary: u64) -> String
{
    let mut bi = binary;
    let mut sum = 0;
    let mut i = 0;

    // Conversion binaire -> décimal
    while bi != 0 {
        let rem = bi % 10;
        bi /= 10;
        sum += rem * 2_u64.pow(i);
        i += 1;
    }

    // Conversion décimal -> hexadécimal
    let mut remain = Vec::new();
    while sum != 0 {
        remain.push(sum % 16);
        sum /= 16;
    }

    // Création de la chaîne hexadécimale
    let hex_string: String = remain.iter().rev().map(|&digit| {
        match digit {
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => std::char::from_digit(digit as u32, 10).unwrap(),
        }
    }).collect();

    hex_string
}


// fonctions conversions octal → binary
pub fn octal_to_binary (octal: u64) -> String
{
    let decimal_value = u64::from_str_radix(&format!("{}", octal), 8).unwrap();
    return format!("{:b}", decimal_value);
}

// fonctions conversions binaire → décimal
pub fn octal_to_decimal (octal: u64) -> String
{
    let octal_str : String = octal.to_string();
    let decimal_value = u64::from_str_radix(&octal_str, 8).unwrap();
    return decimal_value.to_string();
}

// fonctions conversions binaire → décimal
pub fn octal_to_hexadecimal (octal: u64) -> String
{
    let decimal_value = u64::from_str_radix(&format!("{}", octal), 8).unwrap();
    return format!("{:X}", decimal_value);
}


// fonctions conversions décimal → binaire
pub fn decimal_to_binary (decimal : u64) -> String
{
    return format!("{:b}", decimal);

}

// fonctions conversions décimal → octal
pub fn decimal_to_octal (decimal : u64) -> String
{
    return format!("{:o}", decimal);

}

// fonctions conversions décimal → hexadécimal
pub fn decimal_to_hexadecimal (decimal : u64) -> String
{
    return format!("{:X}", decimal);

}


// fonctions conversions hexadécimal → binaire
pub fn hexadecimal_to_binary (hexadecimal : &str) -> String
{
    let decimal = u64::from_str_radix(hexadecimal, 16).unwrap();
    return format!("{:b}", decimal);
}

// fonctions conversions hexadécimal → octal
pub fn hexadecimal_to_octal (hexadecimal : &str) -> String
{
    let decimal = u64::from_str_radix(hexadecimal, 16).unwrap();
    return format!("{:o}", decimal);
}

// fonctions conversions hexadécimal → décimale
pub fn hexadecimal_to_decimal (hexadecimal : &str) -> String
{
    return u64::from_str_radix(hexadecimal, 16).unwrap().to_string();

}





