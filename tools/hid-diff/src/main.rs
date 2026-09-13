use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "hid-diff",
    about = "Differential packet analysis tool for HID reverse engineering",
    long_about = "Compare two HID packet captures (raw hex files, text lines, or JSON arrays), identifying changed offsets, variable fields, and report ID groupings."
)]
struct Args {
    #[arg(help = "Baseline capture file (File A)")]
    file_a: PathBuf,

    #[arg(help = "Modified capture file (File B)")]
    file_b: PathBuf,

    #[arg(short, long, help = "Group output by Report ID")]
    group_by_report_id: bool,
}

fn load_packets(path: &Path) -> Result<Vec<Vec<u8>>, String> {
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

    // Try parsing as JSON array of hex strings first
    if let Ok(strings) = serde_json::from_str::<Vec<String>>(&content) {
        let mut packets = Vec::new();
        for s in strings {
            if let Ok(bytes) = hex_decode(s.trim()) {
                packets.push(bytes);
            }
        }
        if !packets.is_empty() {
            return Ok(packets);
        }
    }

    // Try line-by-line text parsing
    let mut packets = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        // Skip comments, empty lines, or header texts
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed.starts_with("//")
            || trimmed.contains(':')
        {
            continue;
        }
        if let Ok(bytes) = hex_decode(trimmed) {
            if !bytes.is_empty() {
                packets.push(bytes);
            }
        }
    }

    if !packets.is_empty() {
        return Ok(packets);
    }

    // Fallback to raw binary
    let raw = fs::read(path).map_err(|e| format!("Failed to read {}: {e}", path.display()))?;
    Ok(vec![raw])
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    let clean: String = s
        .chars()
        .filter(|c| !c.is_whitespace() && *c != ',')
        .collect();
    if !clean.len().is_multiple_of(2) {
        return Err(());
    }

    let mut bytes = Vec::with_capacity(clean.len() / 2);
    for i in (0..clean.len()).step_by(2) {
        let byte = u8::from_str_radix(&clean[i..i + 2], 16).map_err(|_| ())?;
        bytes.push(byte);
    }
    Ok(bytes)
}

fn main() -> ExitCode {
    let args = Args::parse();

    let packets_a = match load_packets(&args.file_a) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading File A: {e}");
            return ExitCode::FAILURE;
        }
    };

    let packets_b = match load_packets(&args.file_b) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error loading File B: {e}");
            return ExitCode::FAILURE;
        }
    };

    println!("HID Packet Differential Analysis");
    println!(
        "File A: {} ({} packet(s))",
        args.file_a.display(),
        packets_a.len()
    );
    println!(
        "File B: {} ({} packet(s))",
        args.file_b.display(),
        packets_b.len()
    );
    println!("{:-<60}", "");

    let max_len = packets_a.len().max(packets_b.len());

    for idx in 0..max_len {
        let empty = Vec::new();
        let a = packets_a.get(idx).unwrap_or(&empty);
        let b = packets_b.get(idx).unwrap_or(&empty);

        let report_id_a = a.first().copied();
        let report_id_b = b.first().copied();

        println!(
            "Packet {} (Length A: {} bytes, Length B: {} bytes)",
            idx + 1,
            a.len(),
            b.len()
        );
        if let Some(rid) = report_id_a {
            print!("  Report ID: 0x{:02X}", rid);
            if let Some(rid_b) = report_id_b {
                if rid_b != rid {
                    print!(" (changed to 0x{:02X})", rid_b);
                }
            }
            println!();
        }

        let compare_len = a.len().max(b.len());
        let mut changed_count = 0;
        let mut unchanged_count = 0;
        let mut diffs = Vec::new();

        for offset in 0..compare_len {
            let byte_a = a.get(offset);
            let byte_b = b.get(offset);

            if byte_a != byte_b {
                changed_count += 1;
                diffs.push((offset, byte_a, byte_b));
            } else {
                unchanged_count += 1;
            }
        }

        if diffs.is_empty() {
            println!("  No differences found across {} bytes.\n", unchanged_count);
            continue;
        }

        println!("  Offset   A      B      Note");
        println!("  ---------------------------");
        for (offset, ba, bb) in diffs {
            let val_a = ba
                .map(|v| format!("{:02X}", v))
                .unwrap_or_else(|| "--".into());
            let val_b = bb
                .map(|v| format!("{:02X}", v))
                .unwrap_or_else(|| "--".into());
            println!("  0x{:04X}   {}     {}     changed", offset, val_a, val_b);
        }
        println!();
        println!("  unchanged: {} byte(s)", unchanged_count);
        println!("  changed:   {} byte(s)", changed_count);
        println!("{:-<60}", "");
    }

    ExitCode::SUCCESS
}
