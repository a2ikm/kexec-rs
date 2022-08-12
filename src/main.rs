use clap::Parser;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::{Command, Stdio};

const KUBECTL: &str = "kubectl";

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    app: String,

    commands: Vec<String>,

    #[clap(short, long)]
    namespace: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ret = get_pod(&args);
    if let Some(pod_name) = ret {
        execute(&args, pod_name)
    } else {
        process::exit(1)
    }
}

fn get_pod(args: &Args) -> Option<String> {
    let app_selector = format!("app={}", args.app);
    let mut kubectl_args = vec![
        "get",
        "pods",
        "-o",
        "jsonpath=\"{.items[0].metadata.name}\"",
        "-l",
        &app_selector,
    ];
    if let Some(namespace) = &args.namespace {
        let mut namespace_args = vec!["-n", namespace];
        kubectl_args.append(&mut namespace_args);
    }

    dump_command(&kubectl_args);
    let result = Command::new(KUBECTL)
        .args(kubectl_args)
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

fn execute(args: &Args, pod: String) {
    let mut kubectl_args = vec!["exec", &pod, "-it", "--container", &args.app];
    if let Some(namespace) = &args.namespace {
        let mut namespace_args = vec!["-n", namespace];
        kubectl_args.append(&mut namespace_args);
    }
    kubectl_args.push("--");

    let mut commands = args
        .commands
        .iter()
        .map(|x| -> &str { x.as_str() })
        .collect::<Vec<_>>();
    kubectl_args.append(&mut commands);

    dump_command(&kubectl_args);
    let err = Command::new(KUBECTL).args(kubectl_args).exec();
    panic!("Failed to exec: {}", err);
}

fn dump_command(kubectl_args: &Vec<&str>) {
    println!("$ {} {}", KUBECTL, (*kubectl_args).join(" "));
}

fn unquote(quoted: String) -> String {
    let mut s = String::from(quoted);
    s.remove(0);
    s.remove(s.len() - 1);
    return s;
}
