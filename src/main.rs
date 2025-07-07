use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let report = Report {
        os: String::from(System::name().unwrap_or("unknown".to_string())),
        host: String::from(System::host_name().unwrap_or("unknown".to_string())),
        kernel: String::from(System::kernel_version().unwrap_or("unknown".to_string())),
        uptime: String::from(System::uptime().to_string()),
        packages: String::from("Packages"),
        shell: String::from(std::env::var("SHELL").unwrap_or("unknown".to_string())),
        resolution: String::from("Resolution"),
        de: String::from(std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("unknown".to_string())),
        wm: String::from("WM"),
        term: String::from(std::env::var("TERM").unwrap_or("unknown".to_string())),
        cpu: String::from(sys.cpus()[0].brand()),
        gpu: String::from("GPU"),
        mem: String::from(format!(
            "{}MiB / {}MiB",
            sys.used_memory() / 1000000,
            sys.total_memory() / 1000000
        )),
    };

    print_result(report);
}

struct Report {
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    packages: String,
    shell: String,
    resolution: String,
    de: String,
    wm: String,
    term: String,
    cpu: String,
    gpu: String,
    mem: String,
}

fn print_result(report: Report) {
    println!("OS: {}", report.os);
    println!("Host: {}", report.host);
    println!("Kernel: {}", report.kernel);
    println!("Uptime: {}", report.uptime);
    println!("Packages: {}", report.packages);
    println!("Shell: {}", report.shell);
    println!("Resolution: {}", report.resolution);
    println!("DE: {}", report.de);
    println!("WM: {}", report.wm);
    println!("Terminal: {}", report.term);
    println!("CPU: {}", report.cpu);
    println!("GPU: {}", report.gpu);
    println!("Memory: {}", report.mem);
}
