use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn rotate(original_position : u32, rotation : String) -> u32 {
    let way : char = rotation.chars().nth(0).unwrap();
    let mut number : u32 = rotation[1..].parse::<u32>().unwrap();

    if way == 'L'{
        number = 100 - number % 100;
    } else {
        number = number % 100;
    }

    ( original_position + number ) % 100
}

fn rotate_and_count(original_position : i32, rotation : String) -> (i32, i32) {
    let way : char = rotation.chars().nth(0).unwrap();
    let mut number : i32 = rotation[1..].parse::<i32>().unwrap();
    let mut nb_clicks : i32 = number / 100;

    if way == 'L'{
        number = number % 100;
        if original_position != 0 && (original_position - number) <= 0 {
            nb_clicks += 1;
        }

        ((original_position + 100 - number) % 100, nb_clicks)
    } else {
        number = number % 100;
        if (original_position + number) >= 100 {
            nb_clicks += 1;
        }
        (( original_position + number ) % 100, nb_clicks)
    }

}

fn retrieve_password(filename : String) -> u32 {
    let mut position : u32 = 50;
    let mut password : u32 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            position = rotate(position, line);
            if position == 0 {
                password = password + 1;
            }
        }
    }

    password
}

fn retrieve_0x4protocol(filename : String) -> u32 {
    let mut position : i32 = 50;
    let mut password : u32 = 0;

    if let Ok(lines) = read_lines(filename) {
        for line in lines.map_while(Result::ok) {
            let nb_clicks;
            (position, nb_clicks) = rotate_and_count(position, line);
            password += nb_clicks as u32;
        }
    }

    password
}

fn main() {
    let mut password = retrieve_password(String::from("input"));
    println!("The password is : {}.", password);
    password = retrieve_0x4protocol(String::from("input"));
    println!("The new password is : {}.", password);
    
    /* let (mut original_position, mut nb_clicks) = rotate_and_count(50, String::from("R1050"));
    println!("new position : {}, 11 nb_clicks : {}", original_position, nb_clicks);

    (original_position, nb_clicks) = rotate_and_count(50, String::from("L1050"));
    println!("new position : {}, 11 nb_clicks : {}", original_position, nb_clicks);

    (original_position, nb_clicks) = rotate_and_count(1, String::from("L3"));
    println!("98 new position : {}, 1 nb_clicks : {}", original_position, nb_clicks);

    (original_position, nb_clicks) = rotate_and_count(99, String::from("R102"));
    println!("1 new position : {}, 2 nb_clicks : {}", original_position, nb_clicks);

    (original_position, nb_clicks) = rotate_and_count(50, String::from("R1051"));
    println!("new position : {}, 11 nb_clicks : {}", original_position, nb_clicks);

    (original_position, nb_clicks) = rotate_and_count(0, String::from("L100"));
    println!("0 new position : {}, 1 nb_clicks : {}", original_position, nb_clicks); */
} 
