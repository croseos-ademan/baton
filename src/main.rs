mod process;

use std::process as std_process;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: baton <command> [args...]");
        eprintln!("       baton --pid <pid> <command> [args...]");
        std_process::exit(1);
    }

    let (target_pid, cmd_start) = if args[1] == "--pid" {
        if args.len() < 4 {
            eprintln!("Error: --pid requires a PID and a command");
            std_process::exit(1);
        }
        let pid: u32 = args[2].parse().unwrap_or_else(|_| {
            eprintln!("Error: invalid PID '{}'", args[2]);
            std_process::exit(1);
        });
        (Some(pid), 3)
    } else {
        (None, 1)
    };

    let command = &args[cmd_start];
    let cmd_args = &args[cmd_start + 1..];

    match process::handoff(target_pid, command, cmd_args) {
        Ok(new_pid) => {
            println!("baton: handed off to new process (pid={})", new_pid);
        }
        Err(e) => {
            eprintln!("baton: handoff failed: {}", e);
            std_process::exit(1);
        }
    }
}
