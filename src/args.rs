pub struct Args {
  pub interval: u64,
}

impl Args {
  pub fn parse() -> Args {
    let env_args = std::env::args().collect::<Vec<String>>();
    let mut args = Args{interval: 5};

    for i in 0..env_args.len() {
      if env_args[i] == String::from("-i") {
        if i + 1 < env_args.len() {
          args.interval = env_args[i + 1].parse::<u64>().unwrap();
        }
      }
    }

    return args;
  }
}