use serde::Serialize;
use std::path::Path;
use sysinfo::{Disks, System};

const GIB: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Clone, Serialize)]
pub struct SystemHealthReport {
    pub logical_cpu_count: usize,
    pub total_memory_bytes: u64,
    pub available_memory_bytes: u64,
    pub disk_total_bytes: Option<u64>,
    pub disk_available_bytes: Option<u64>,
}

impl SystemHealthReport {
    pub fn total_memory_gib(&self) -> f64 {
        self.total_memory_bytes as f64 / GIB as f64
    }

    pub fn available_memory_gib(&self) -> f64 {
        self.available_memory_bytes as f64 / GIB as f64
    }

    pub fn disk_total_gib(&self) -> Option<f64> {
        self.disk_total_bytes.map(|bytes| bytes as f64 / GIB as f64)
    }

    pub fn disk_available_gib(&self) -> Option<f64> {
        self.disk_available_bytes
            .map(|bytes| bytes as f64 / GIB as f64)
    }
}

pub fn check_system_health() -> SystemHealthReport {
    let system = System::new_all();

    let logical_cpu_count = system.cpus().len();
    let total_memory_bytes = system.total_memory();
    let available_memory_bytes = system.available_memory();

    let disks = Disks::new_with_refreshed_list();

    // Prefer the filesystem containing the current working directory.
    let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new("/").to_path_buf());

    let disk = disks
        .list()
        .iter()
        .filter(|disk| current_dir.starts_with(disk.mount_point()))
        .max_by_key(|disk| disk.mount_point().components().count())
        .or_else(|| {
            disks
                .list()
                .iter()
                .find(|disk| disk.mount_point() == Path::new("/"))
        });

    let (disk_total_bytes, disk_available_bytes) = match disk {
        Some(disk) => (Some(disk.total_space()), Some(disk.available_space())),
        None => (None, None),
    };

    SystemHealthReport {
        logical_cpu_count,
        total_memory_bytes,
        available_memory_bytes,
        disk_total_bytes,
        disk_available_bytes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gib_conversions_work() {
        let report = SystemHealthReport {
            logical_cpu_count: 8,
            total_memory_bytes: 64 * GIB,
            available_memory_bytes: 32 * GIB,
            disk_total_bytes: Some(2 * 1024 * GIB),
            disk_available_bytes: Some(500 * GIB),
        };

        assert_eq!(report.total_memory_gib(), 64.0);
        assert_eq!(report.available_memory_gib(), 32.0);
        assert_eq!(report.disk_total_gib(), Some(2048.0));
        assert_eq!(report.disk_available_gib(), Some(500.0));
    }
}
