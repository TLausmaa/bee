use std::process::exit;

pub struct Args {
  pub interval: u64,
  pub threshold: f64,
  pub verbose: bool,
}

fn print_help() {
  println!("Usage: bee [options]");
  println!("Options:");
  println!("  -i <interval>  Polling interval in seconds (default: 5)");
  println!("  -t <threshold> Alert threshold in percentage (default: 70.0)");
  println!("  -v             Verbose mode");
  std::process::exit(0);
}

impl Args {
  pub fn parse() -> Args {
    let env_args = std::env::args().collect::<Vec<String>>();
    let mut args = Args{
      interval: 5, 
      threshold: 70.0, 
      verbose: false, 
    };

    for i in 0..env_args.len() {
      if env_args[i] == String::from("-i") {
        if i + 1 < env_args.len() {
          args.interval = env_args[i + 1].parse::<u64>().unwrap();
        }
      }
      if env_args[i] == String::from("-t") {
        if i + 1 < env_args.len() {
          args.threshold = env_args[i + 1].parse::<f64>().unwrap();
        }
      }
      if env_args[i] == String::from("-v") {
        args.verbose = true;
      }
      if env_args[i] == String::from("-h") || env_args[i] == String::from("help") || env_args[i] == String::from("--help") {
        print_help();
        exit(0);
      }
    }

    return args;
  }
}