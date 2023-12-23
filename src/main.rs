use clap::{crate_description, crate_name, crate_version, Arg, Command};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::{spawn, sleep}, time::Duration,
    process::{Command as StdCommand}
};

fn main() {
    let num_cpus_string = num_cpus::get().to_string();
    let matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .arg({
            Arg::new("threads")
                .help("Number of threads for lookup")
                .short('t')
                .long("threads")
                .default_value(num_cpus_string)
        })
        .get_matches();

    let threads = matches
        .get_one::<String>("threads")
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let exit_flag = Arc::new(AtomicBool::new(false));

    let threads = (0..threads)
        .map(|_| {
            let exit_flag = Arc::clone(&exit_flag);
            spawn(move || {
                while !exit_flag.load(Ordering::Relaxed) {
                    run();
                    sleep(Duration::from_millis(10000));
                }
            })
        })
        .collect::<Vec<_>>();

    for thread in threads {
        thread.join().unwrap();
    }
}

fn run() {
    let output = StdCommand::new("aptos")
    .arg("move")
    .arg("run")
    .arg("--function-id")
    .arg("0x1fc2f33ab6b624e3e632ba861b755fd8e61d2c2e6cf8292e415880b4c198224d::apts::mint")
    .arg("--args")
    .arg("string:APTS")
    .arg("--expiration-secs")
    .arg("120")
    .arg("--assume-yes")
    .output()
    .unwrap();
    let out = String::from_utf8(output.stdout).unwrap();
    print!("{:?}", out);
}
