use std::{io, collections, cmp::Ordering};

fn main() {
    println!("Job scheduler test.");

    let mut job_list: Vec<Job> = Vec::new();

    loop {
        println!("Choose your option:");
        println!("a: Add job");
        println!("d: Delete job");
        println!("v: View job list");
        println!("s: Single-step job processing");
        println!("r: Run job processing to completion");
        println!("q: Quit the program");

        let choice = get_text_input();

        match choice.trim().to_lowercase().as_str() {
            "a" => {
                job_list.push(create_job());
            },
            _ => println!("Invalid choice.")
        }
    }
}

fn get_text_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn get_integer_input() -> u32 {
    loop {
        let text = get_text_input();
        match text.trim().parse() {
            Ok(num) => return num,
            Err(_) => print!("Invalid number. Try again: "),
        };
    }
}

fn create_job() -> Job {
    print!("Enter the number of cycles the job needs to complete: ");
    let time = get_integer_input();

    let job = Job {
        total_time: time,
        current_time: 0,
    };

    job
}

struct Job {
    total_time: u32, 
    current_time: u32,
}