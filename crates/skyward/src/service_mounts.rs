//! Filesystems retained by a service's private mount namespace, even when
//! the service has no file open or mapped on them.

use std::collections::BTreeSet;
use std::fs;
use std::io;

/// Device numbers mounted in a namespace other than the recovery's own.
/// The shared namespace is excluded: its mounts leave with the recovery's.
/// Failure to inspect a live service stops detachment rather than treating
/// a filesystem whose references are unknown as unused.
pub fn held(pid: u32) -> io::Result<BTreeSet<(u32, u32)>> {
    let ours = fs::read_link("/proc/self/ns/mnt")?;
    let theirs = fs::read_link(format!("/proc/{pid}/ns/mnt"))?;
    if ours == theirs {
        return Ok(BTreeSet::new());
    }
    devices(&fs::read_to_string(format!("/proc/{pid}/mountinfo"))?)
}

fn devices(table: &str) -> io::Result<BTreeSet<(u32, u32)>> {
    table.lines().map(|line| {
        let invalid = || io::Error::new(io::ErrorKind::InvalidData, "invalid device number in service mountinfo");
        let number = line.split_whitespace().nth(2).ok_or_else(invalid)?;
        let (major, minor) = number.split_once(':').ok_or_else(invalid)?;
        Ok((major.parse().map_err(|_| invalid())?, minor.parse().map_err(|_| invalid())?))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bind_alias_keeps_the_same_device_without_any_open_file() {
        let table = "10 1 254:8 / /mnt/mi_ext ro - erofs /dev/block/dm-8 ro\n\
                     11 1 254:8 /firmware /vendor/firmware ro - erofs /dev/mapper/mi_ext_b ro\n\
                     12 1 254:3 / /mnt/system ro - erofs /dev/block/dm-3 ro\n";
        assert_eq!(devices(table).unwrap(), BTreeSet::from([(254, 3), (254, 8)]));
    }

    #[test]
    fn a_mount_path_with_escaped_spaces_does_not_change_its_device() {
        assert_eq!(devices(r"10 1 254:8 / /mnt/a\040b ro - erofs /dev/block/dm-8 ro").unwrap(), BTreeSet::from([(254, 8)]));
    }

    #[test]
    fn a_broken_table_is_not_treated_as_an_unused_filesystem() {
        for table in ["10 1", "10 1 invalid / /mnt ro", "10 1 254:x / /mnt ro"] {
            assert_eq!(devices(table).unwrap_err().kind(), io::ErrorKind::InvalidData);
        }
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn our_own_namespace_does_not_pin_all_our_mounts() {
        assert!(held(std::process::id()).unwrap().is_empty());
    }
}
