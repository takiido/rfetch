use crate::colors;

pub struct Report {
    pub os: String,
    pub host: String,
    pub kernel: String,
    pub uptime: String,
    pub packages: String,
    pub shell: String,
    pub resolution: String,
    pub de: String,
    pub wm: String,
    pub term: String,
    pub cpu: String,
    pub gpu: String,
    pub mem: String,
}

impl Report {
    pub fn new(
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
    ) -> Self {
        Self {
            os,
            host,
            kernel,
            uptime,
            packages,
            shell,
            resolution,
            de,
            wm,
            term,
            cpu,
            gpu,
            mem,
        }
    }

    pub fn print(&self) {
        println!("{}", Self::format_entry("OS", &self.os));
        println!("{}", Self::format_entry("Host", &self.host));
        println!("{}", Self::format_entry("Kernel", &self.kernel));
        println!("{}", Self::format_entry("Uptime", &self.uptime));
        println!("{}", Self::format_entry("Packages", &self.packages));
        println!("{}", Self::format_entry("Shell", &self.shell));
        println!("{}", Self::format_entry("Resolution", &self.resolution));
        println!("{}", Self::format_entry("DE", &self.de));
        println!("{}", Self::format_entry("WM", &self.wm));
        println!("{}", Self::format_entry("Terminal", &self.term));
        println!("{}", Self::format_entry("CPU", &self.cpu));
        println!("{}", Self::format_entry("GPU", &self.gpu));
        println!("{}", Self::format_entry("Memory", &self.mem));
    }

    fn format_entry(key: &str, value: &str) -> String {
        let formatted_str = format!("\x1b[1m{}{}: {}{}", colors::YELLOW, key, colors::NC, value);
        return formatted_str;
    }
}
