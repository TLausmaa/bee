use std::{fs, process::Command};

struct MemInfo {
    total_kb: u64,
    free_kb: u64,
    available_kb: u64,
}

fn main() {
    let mem_info = read_mem_info();
    println!("Total: {} kB", mem_info.total_kb);
    println!("Avail: {} kB", mem_info.available_kb);
    println!("Free: {} kB", mem_info.free_kb);
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
