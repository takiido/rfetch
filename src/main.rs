mod colors;
mod report;

use report::Report;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let report = Report::new(
        String::from(System::name().unwrap_or("unknown".to_string())),
        String::from(System::host_name().unwrap_or("unknown".to_string())),
        String::from(System::kernel_version().unwrap_or("unknown".to_string())),
        String::from(System::uptime().to_string()),
        String::from("Packages"),
        String::from(std::env::var("SHELL").unwrap_or("unknown".to_string())),
        String::from("Resolution"),
        String::from(std::env::var("XDG_CURRENT_DESKTOP").unwrap_or("unknown".to_string())),
        String::from("WM"),
        String::from(std::env::var("TERM").unwrap_or("unknown".to_string())),
        String::from(sys.cpus()[0].brand()),
        String::from("GPU"),
        String::from(format!(
            "{}MiB / {}MiB ({:.1}%)",
            sys.used_memory() / 1048576,
            sys.total_memory() / 1048576,
            sys.used_memory() as f64 / sys.total_memory() as f64 * 100.0
        )),
    );

    Report::print(&report);
}
