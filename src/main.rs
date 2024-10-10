use std::{thread, fs, process::Command, time};
use chrono::prelude::*;

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

mod args;
mod config;

struct MemInfo {
    total_kb: u64,
    free_kb: u64,
    available_kb: u64,
}

struct RuntimeCache {
    last_alert: Option<DateTime<Utc>>,
}

fn main() {
    let args = args::Args::parse();
    let mut runtime_cache = RuntimeCache { last_alert: None };

    println!("Bee is watching memory usage now.");
    println!("Polling interval: {} seconds", args.interval);
    println!("Alert threshold: {:.2}%", args.threshold);

    loop {
        thread::sleep(time::Duration::from_secs(args.interval));
        let mem_info = read_mem_info();
        analyze_mem(mem_info, &args, &mut runtime_cache);
    }
}

fn analyze_mem(mem_info: MemInfo, args: &args::Args, runtime_cache: &mut RuntimeCache) {
    let used_kb = mem_info.total_kb - mem_info.available_kb;
    let used_percent = (used_kb as f64 / mem_info.total_kb as f64) * 100.0;

    if args.verbose {
        println!("Total: {} KB, Free: {} KB, Available: {} KB, Used: {} KB, Used%: {:.2}%", 
            mem_info.total_kb, mem_info.free_kb, mem_info.available_kb, used_kb, used_percent);
    }

    if used_percent >= args.threshold {
        if args.verbose {
            send_email(used_percent, runtime_cache);
        }
    }
}

fn read_mem_info() -> MemInfo {
    let os = std::env::consts::OS;
    return match os {
        "linux" => read_mem_linux(),
        "freebsd" => read_mem_freebsd(),
        "macos" => read_mem_macos(),
        _ => {
            println!("Unsupported OS: {}", os);
            MemInfo {
                total_kb: 0,
                free_kb: 0,
                available_kb: 0,
            }
        }
    };    
}

fn read_mem_linux() -> MemInfo {
    let mut mem_info = MemInfo {
        total_kb: 0,
        free_kb: 0,
        available_kb: 0,
    };

    let filename = "/proc/meminfo";
    let str = fs::read_to_string(filename).expect("Failed to read /proc/meminfo");
    
    str.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            if parts[0] == "MemTotal:" {
                mem_info.total_kb = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "MemFree:" {
                mem_info.free_kb = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "MemAvailable:" {
                mem_info.available_kb = parts[1].parse::<u64>().unwrap();
            } 
        }
    });
    
    return mem_info;
}

fn read_mem_freebsd() -> MemInfo {
    let mut mem_info = MemInfo {
        total_kb: 0,
        free_kb: 0,
        available_kb: 0,
    };

    let output = Command::new("/sbin/sysctl")
        .arg("-a")
        .output()
        .expect("Failed to execute sysctl command");

    if !output.status.success() {
        return mem_info;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut page_size: u64 = 0;
    let mut mem_inactive: u64 = 0;
    let mut mem_cache: u64 = 0;
    let mut mem_free: u64 = 0;

    stdout.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if parts[0] == "vm.stats.vm.v_inactive_count:" {
                mem_inactive = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "vm.stats.vm.v_cache_count:" {
                mem_cache = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "vm.stats.vm.v_free_count:" {
                mem_free = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "hw.pagesize:" {
                page_size = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "hw.physmem:" {
                mem_info.total_kb = parts[1].parse::<u64>().unwrap() / 1000;
            }
        }
    });

    mem_info.available_kb = ((mem_inactive + mem_cache + mem_free) * page_size) / 1000;
    mem_info.free_kb = (mem_free * page_size) / 1000;
    // used = total_kb - available_kb
    
    return mem_info;
}

fn read_mem_macos() -> MemInfo {
    println!("Not implemented: MacOS");
    return MemInfo {
        total_kb: 0,
        free_kb: 0,
        available_kb: 0,
    };
}

fn send_email(used_percent: f64, runtime_cache: &mut RuntimeCache) {
    let datetime = Utc::now();
    
    if runtime_cache.last_alert != None {
        let last_alert = runtime_cache.last_alert.unwrap();
        let duration = datetime.signed_duration_since(last_alert).num_minutes();
        if duration < 60 {
            return;
        }
    }

    let config = config::Config::read().email;
    let operating_system = std::env::consts::OS;
    let subject = format!("Bee alert 🐝: high memory usage on {}", operating_system);
    let body = format!(
        r#"Memory usage on {} exceeded alerting threshold {:.2}% on {}.
Please check the system."#, operating_system, used_percent, datetime);

    let email = Message::builder()
        .from(config.from.parse().unwrap())
        .reply_to(config.from.parse().unwrap())
        .to(config.to.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .unwrap();

    let creds = Credentials::new(
        config.smtp_email.to_owned(), 
        config.smtp_password.to_owned()
    );

    let mailer = SmtpTransport::relay(&config.smtp_server)
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            runtime_cache.last_alert = Some(datetime);
        },
        Err(e) => {
            eprintln!("Could not send email alert: {e:?}");
        },
    }
}
