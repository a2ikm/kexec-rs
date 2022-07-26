use clap::Parser;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::{Command, Stdio};

const KUBECTL: &str = "kubectl";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    app: String,
}

fn main() {
    let args = Args::parse();

    let ret = get_pod(&args.app);
    if let Some(pod_name) = ret {
        execute(&args.app, pod_name)
    } else {
        process::exit(1)
    }
}

fn get_pod(app: &String) -> Option<String> {
    let app_selector = format!("app={}", app);
    let args = vec![
        "get",
        "pods",
        "-o",
        "jsonpath=\"{.items[0].metadata.name}\"",
        "-l",
        &app_selector,
    ];

    dump_command(&args);
    let result = Command::new(KUBECTL)
        .args(args)
        .stderr(Stdio::inherit())
        .output();

    if let Ok(output) = result {
        if output.status.success() {
            return Some(unquote(String::from_utf8(output.stdout).unwrap()));
        } else {
            return None;
        }
    } else {
        return None;
    }
}

fn execute(app: &String, pod: String) {
    // TODO: Handle namespace
    // TODO: Handle given command
    let args = vec![
        "exec",
        &pod,
        "-it",
        "--container",
        app,
        "--",
        "echo",
        "hello",
    ];

    dump_command(&args);
    let err = Command::new(KUBECTL).args(args).exec();
    panic!("Failed to exec: {}", err);
}

fn dump_command(args: &Vec<&str>) {
    println!("$ {} {}", KUBECTL, (*args).join(" "));
}

fn unquote(quoted: String) -> String {
    let mut s = String::from(quoted);
    s.remove(0);
    s.remove(s.len() - 1);
    return s;
}
