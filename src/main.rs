#![allow(unused_must_use)]
extern crate sysinfo; // importa la libreria, al no ser local hay que usar 'extern crate'
use chrono::{Local, NaiveDateTime};
use sysinfo::{Disks, System}; // importa System de sysinfo
mod windows;

async fn display() {
    let shell: String = windows::fetch_latest_ps_version();
    let windoww: (
        windows::Win32_OperatingSystem,
        windows::Win32_Processor,
        windows::Win32_VideoController,
        Vec<windows::Win32_LogicalDisk>,
    ) = windows::fetch().await;
    let uptime: chrono::TimeDelta = Local::now().naive_local().signed_duration_since(
        NaiveDateTime::parse_from_str(
            &windoww.0.last_boot_up_time[..windoww.0.last_boot_up_time.len() - 4],
            "%Y%m%d%H%M%S%.6f",
        )
        .unwrap(),
    );
    let info: os_info::Info = os_info::get();
    let binding: Option<&str> = info.edition();
    let osname: &str = binding.as_deref().expect("wtf");
    let binding: Option<&str> = info.architecture();
    let architecture: &str = binding.as_deref().expect("wtf");
    let binding: Option<String> = System::host_name();
    let hostname: &str = binding.as_deref().expect("wtf");
    let mut sys: System = System::new_all(); // Esto se usa para loguear todos los datos
    let mut disks_info: String = String::new();
    let disks: Disks = Disks::new_with_refreshed_list();
    sys.refresh_all(); // Esto es para renovar los datos al ultimo momento

    println!("Host       : {}", hostname);
    println!("OS         : {} {}", osname, architecture);
    println!(
        "Kernel     : {}",
        info.version().to_string().trim_matches('"')
    );
    println!("Shell      : {}", shell);
    println!(
        "Uptime     : {} hs, {} min",
        uptime.num_hours(),
        uptime.num_minutes() % 60
    );
    println!(
        "Resolution : {}x{} | {}hz",
        &windoww.2.current_horizontal_resolution,
        &windoww.2.current_vertical_resolution,
        &windoww.2.current_refresh_rate
    );
    println!("GPU        : {} ", &windoww.2.name.to_string());
    println!("CPU        : {}", &windoww.1.name.to_string());
    println!(
        "Memory     : {} MB / {} MB",
        sys.used_memory() / 1024 / 1024,
        sys.total_memory() / 1024 / 1024
    );

    for (index, disk) in disks.iter().enumerate() {
        if index > 0 {
            disks_info.push_str(" | ");
        }
        let mount: std::borrow::Cow<str> = disk.mount_point().to_string_lossy();
        let mount2: String = mount.to_string().replace(r"\", "");
        let space: u64 = disk.total_space() / 1024 / 1024 / 1024;
        disks_info.push_str(&format!("({}) {} GB", mount2, space));
    }
    println!("Disk       : {}", disks_info);
}

#[tokio::main]
async fn main() {
    display().await;
}
