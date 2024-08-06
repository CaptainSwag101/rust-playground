use std::{io, io::Write, option, collections, cmp::Ordering};

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
                println!("Job added.")
            },
            "s" => _ = process_job_list(&mut job_list),
            "r" => {
                while !process_job_list(&mut job_list) { }
                println!("Job processing completed.");
            }
            "q" => break,
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
            Err(_) => {
                print!("Invalid number. Try again: ");
                io::stdout().flush().expect("Unable to flush stdout!");
            },
        };
    }
}

fn create_job() -> Job {
    print!("Enter the number of cycles the job needs to complete: ");
    io::stdout().flush().expect("Unable to flush stdout!");
    let time = get_integer_input();

    print!("Enter the priority of the job: ");
    io::stdout().flush().expect("Unable to flush stdout!");
    let prio = get_integer_input();

    let job = Job {
        total_time: time,
        current_time: 0,
        priority: prio,
        sleep: false,
    };

    job
}

fn process_job_list(job_list: &mut Vec<Job>) -> bool {
    // Sort job list by priority
    job_list.sort_by(|a, b| b.priority.cmp(&a.priority));

    // Process the first (highest-priority) job that is not asleep.
    // On the way, if we fail to process any jobs, mark everything as done.
    let mut all_done = true;
    for job in job_list {
        if !job.sleep && job.current_time < job.total_time {
            job.current_time += 1;
            all_done = false;
            if job.current_time == job.total_time {
                println!("Job with priority {} completed processing.", job.priority);
            }
            break;
        }
    }

    all_done
}

struct Job {
    total_time: u32, 
    current_time: u32,
    priority: u32,
    sleep: bool,
}