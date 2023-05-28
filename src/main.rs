use std::time::Instant;
use std::process::Command;

fn time_spend(task: String) {
    let start = Instant::now();

    let _output = Command::new(task)
    .output()
    .expect("failed to execute process");

    let end = Instant::now();
    let duration = end - start;

    println!("{} ", duration.as_micros().to_string());
}

fn main() {
    println!("input `exit` to exit");
    loop {
        println!("input task command: ");
        let mut task = String::new();
        std::io::stdin().read_line(&mut task).expect("failed to read line");
        println!("exec {}", task);
        if task.trim() == "exit" {
            break;
        }

        time_spend(task);
    }
}
