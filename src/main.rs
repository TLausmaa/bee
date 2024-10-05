use std::{fs, process::Command};

struct MemInfo {
    total_kb: u64,
    free_kb: u64,
    available_kb: u64,
}

fn main() {
    let meminfo = read_mem_info();
    println!("Total: {} kB", meminfo.total_kb);
    println!("Avail: {} kB", meminfo.available_kb);
    println!("Free: {} kB", meminfo.free_kb);
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
    let mut meminfo = MemInfo {
        total_kb: 0,
        free_kb: 0,
        available_kb: 0,
    };

    let output = Command::new("/sbin/sysctl")
        .arg("-a")
        .output()
        .expect("Failed to execute sysctl command");

    if !output.status.success() {
        return meminfo;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout.lines().for_each(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            if parts[0] == "MemTotal:" {
                meminfo.total_kb = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "MemFree:" {
                meminfo.free_kb = parts[1].parse::<u64>().unwrap();
            } else if parts[0] == "MemAvailable:" {
                meminfo.available_kb = parts[1].parse::<u64>().unwrap();
            }
        }
    });
    
    return meminfo;
}

fn read_mem_macos() -> MemInfo {
    println!("Not implemented: MacOS");
    return MemInfo {
        total_kb: 0,
        free_kb: 0,
        available_kb: 0,
    };
}