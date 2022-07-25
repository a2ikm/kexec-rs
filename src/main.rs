use std::os::unix::process::CommandExt;
use std::process;
use std::process::{Command, Stdio};

const KUBECTL: &str = "kubectl";
// TODO: Handle given app name
const APP: &str = "app";

fn main() {
    let ret = get_pod(APP.to_string());
    if let Some(pod_name) = ret {
        execute(pod_name)
    } else {
        process::exit(1)
    }
}

fn get_pod(app: String) -> Option<String> {
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

fn execute(pod: String) {
    // TODO: Handle namespace
    // TODO: Handle given command
    let args = vec![
        "exec",
        &pod,
        "-it",
        "--container",
        APP,
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
    println!("{}", quoted);
    let mut s = String::from(quoted);
    s.remove(0);
    s.remove(s.len() - 1);
    return s;
}
