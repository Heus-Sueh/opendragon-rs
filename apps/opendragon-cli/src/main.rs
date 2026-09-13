use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use opendragon_core::actuation::ActuationPoint;
use opendragon_core::color::RgbColor;
use opendragon_core::error::OpenDragonError;
use opendragon_core::lighting::{LightingConfig, LightingMode};
use opendragon_core::polling_rate::PollingRate;
use opendragon_core::rapid_trigger::RapidTriggerConfig;
use opendragon_devices::registry::DeviceRegistry;
use opendragon_hid::discovery::{DeviceEnumerator, DiscoveredDevice};
use opendragon_hid::probe::DeviceProber;

#[derive(Parser)]
#[command(
    name = "opendragon",
    version = "0.2.0",
    about = "Modern extensible configuration utility for Redragon peripherals",
    long_about = "OpenDragon provides safe hardware discovery, non-destructive probing, reverse engineering tools, and configuration for Redragon keyboards, mice, and accessories."
)]
struct Cli {
    #[arg(short, long, global = true, help = "Output response in JSON format")]
    json: bool,

    #[arg(long, global = true, help = "Target device Vendor ID (hex, e.g. 0c45)")]
    vid: Option<String>,

    #[arg(
        long,
        global = true,
        help = "Target device Product ID (hex, e.g. 5004)"
    )]
    pid: Option<String>,

    #[arg(
        long,
        global = true,
        help = "Explicitly acknowledge and permit dangerous low-level hardware operations"
    )]
    i_know_what_im_doing: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "List connected Redragon and HID peripherals")]
    List {
        #[arg(
            short,
            long,
            help = "List all connected HID devices, not just Redragon"
        )]
        all: bool,
    },

    #[command(about = "Show detailed hardware descriptors, interfaces, and endpoints")]
    Info,

    #[command(about = "Safely and non-destructively probe the target peripheral")]
    Probe {
        #[arg(short, long, help = "Path to write structured JSON probe capture")]
        output: Option<PathBuf>,
    },

    #[command(about = "Configure RGB lighting effects, colors, and brightness")]
    Rgb {
        #[command(subcommand)]
        action: RgbCommands,
    },

    #[command(about = "Configure switch actuation point (magnetic / Hall Effect)")]
    Actuation {
        #[command(subcommand)]
        action: ActuationCommands,
    },

    #[command(about = "Configure Rapid Trigger dynamic reset")]
    RapidTrigger {
        #[command(subcommand)]
        action: RapidTriggerCommands,
    },

    #[command(about = "Configure USB polling rate")]
    PollingRate {
        #[command(subcommand)]
        action: PollingRateCommands,
    },

    #[command(about = "Check /dev/hidraw permissions and show udev setup instructions")]
    Permissions,
}

#[derive(Subcommand)]
enum RgbCommands {
    #[command(about = "Set static color (e.g. '#ff0000' or '00ff00')")]
    Static {
        #[arg(help = "Color in hex format (#RRGGBB)")]
        color: String,
    },

    #[command(about = "Set brightness (0-100)")]
    Brightness {
        #[arg(help = "Brightness percentage (0 to 100)")]
        level: u8,
    },

    #[command(about = "Set lighting mode (static, breathing, rainbow, wave, reactive, off)")]
    Mode {
        #[arg(help = "Lighting mode name")]
        mode: String,
        #[arg(short, long, default_value = "#ffffff", help = "Optional effect color")]
        color: String,
        #[arg(short, long, default_value_t = 100, help = "Brightness (0-100)")]
        brightness: u8,
        #[arg(short, long, default_value_t = 50, help = "Speed (0-100)")]
        speed: u8,
    },
}

#[derive(Subcommand)]
enum ActuationCommands {
    #[command(about = "Get current actuation point")]
    Get,
    #[command(about = "Set actuation point in mm (e.g. 1.2)")]
    Set {
        #[arg(help = "Distance in millimeters (0.1 - 4.0)")]
        distance_mm: f32,
    },
}

#[derive(Subcommand)]
enum RapidTriggerCommands {
    #[command(about = "Show current Rapid Trigger status")]
    Status,
    #[command(about = "Enable Rapid Trigger")]
    Enable {
        #[arg(short, long, default_value_t = 0.2, help = "Press sensitivity in mm")]
        press_mm: f32,
        #[arg(short, long, default_value_t = 0.2, help = "Release sensitivity in mm")]
        release_mm: f32,
    },
    #[command(about = "Disable Rapid Trigger")]
    Disable,
}

#[derive(Subcommand)]
enum PollingRateCommands {
    #[command(about = "Get current polling rate")]
    Get,
    #[command(about = "Set polling rate in Hz (125, 250, 500, 1000, 2000, 4000, 8000)")]
    Set {
        #[arg(help = "Rate in Hz (e.g. 1000)")]
        rate: String,
    },
}

fn parse_hex_u16(s: &str) -> Result<u16, OpenDragonError> {
    let clean = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    u16::from_str_radix(clean, 16)
        .map_err(|e| OpenDragonError::InvalidData(format!("Invalid hex string '{s}': {e}")))
}

fn find_target_device(
    cli_vid: &Option<String>,
    cli_pid: &Option<String>,
) -> Result<DiscoveredDevice, OpenDragonError> {
    let target_vid = cli_vid.as_deref().map(parse_hex_u16).transpose()?;
    let target_pid = cli_pid.as_deref().map(parse_hex_u16).transpose()?;

    let redragon_devices = DeviceEnumerator::enumerate_redragon()?;

    if let (Some(vid), Some(pid)) = (target_vid, target_pid) {
        if let Some(dev) = redragon_devices
            .into_iter()
            .find(|d| d.info.vendor_id == vid && d.info.product_id == pid)
        {
            return Ok(dev);
        }
        // Check all HID devices as fallback
        let all_devices = DeviceEnumerator::enumerate_all()?;
        if let Some(dev) = all_devices
            .into_iter()
            .find(|d| d.info.vendor_id == vid && d.info.product_id == pid)
        {
            return Ok(dev);
        }
        return Err(OpenDragonError::DeviceNotFound(format!(
            "Device with VID {vid:#06x} and PID {pid:#06x} not found"
        )));
    }

    if let Some(first) = redragon_devices.into_iter().next() {
        return Ok(first);
    }

    Err(OpenDragonError::DeviceNotFound(
        "No Redragon peripheral detected. Connect your device or run 'opendragon list --all' to inspect all HID devices.".into(),
    ))
}

fn main() -> ExitCode {
    // Initialize logging from RUST_LOG environment variable (e.g. RUST_LOG=debug)
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    tracing_subscriber::fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match run_cli(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            if let OpenDragonError::PermissionDenied { path } = &err {
                eprintln!(
                    "\nPermission notice: Cannot open device at {}",
                    path.display()
                );
                eprintln!(
                    "To configure permissions without root, install the OpenDragon udev rules:"
                );
                eprintln!("  sudo cp udev/70-opendragon.rules /etc/udev/rules.d/");
                eprintln!("  sudo udevadm control --reload-rules && sudo udevadm trigger");
                eprintln!("Run 'opendragon permissions' for more details.");
            }
            ExitCode::FAILURE
        }
    }
}

fn run_cli(cli: Cli) -> Result<(), OpenDragonError> {
    match cli.command {
        Commands::List { all } => {
            let devices = if all {
                DeviceEnumerator::enumerate_all()?
            } else {
                DeviceEnumerator::enumerate_redragon()?
            };

            if cli.json {
                let infos: Vec<_> = devices.iter().map(|d| &d.info).collect();
                println!("{}", serde_json::to_string_pretty(&infos)?);
                return Ok(());
            }

            if devices.is_empty() {
                println!(
                    "No {}peripherals detected.",
                    if all { "" } else { "Redragon " }
                );
                println!("Try running with '--all' or check 'opendragon permissions'.");
                return Ok(());
            }

            println!(
                "Found {} {}device(s):",
                devices.len(),
                if all { "HID " } else { "Redragon " }
            );
            println!("{:-<78}", "");
            for (idx, dev) in devices.iter().enumerate() {
                let known = DeviceRegistry::find_known(dev.info.vendor_id, dev.info.product_id);
                let title = known
                    .map(|k| k.marketing_name)
                    .unwrap_or(dev.info.product_name.as_str());

                println!(
                    "[{}] {} (VID: 0x{:04x}, PID: 0x{:04x})",
                    idx + 1,
                    title,
                    dev.info.vendor_id,
                    dev.info.product_id
                );
                println!("    Manufacturer : {}", dev.info.vendor_name);
                println!("    Product      : {}", dev.info.product_name);
                if let Some(serial) = &dev.info.serial_number {
                    println!("    Serial       : {}", serial);
                }
                println!("    Interfaces   : {}", dev.info.interfaces.len());
                for iface in &dev.info.interfaces {
                    let path_str = iface.hidraw_path.as_deref().unwrap_or("none");
                    let up_str = iface
                        .usage_page
                        .map(|u| format!("0x{:04x}", u))
                        .unwrap_or_else(|| "Unknown".into());
                    let u_str = iface
                        .usage
                        .map(|u| format!("0x{:04x}", u))
                        .unwrap_or_else(|| "Unknown".into());
                    println!(
                        "      - Interface {}: UsagePage: {}, Usage: {}, Node: {}",
                        iface.interface_number, up_str, u_str, path_str
                    );
                }
                println!();
            }
            Ok(())
        }

        Commands::Info => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&discovered.info)?);
                return Ok(());
            }

            let known =
                DeviceRegistry::find_known(discovered.info.vendor_id, discovered.info.product_id);
            let display_name = known
                .map(|k| k.marketing_name)
                .unwrap_or(discovered.info.product_name.as_str());

            println!("Device: {}", display_name);
            println!("VID: 0x{:04x}", discovered.info.vendor_id);
            println!("PID: 0x{:04x}", discovered.info.product_id);
            if let Some(serial) = &discovered.info.serial_number {
                println!("Serial: {}", serial);
            }
            if let Some(rel) = discovered.info.release_number {
                println!("Release (bcdDevice): 0x{:04x}", rel);
            }
            println!("Interfaces: {}", discovered.info.interfaces.len());
            println!();

            for iface in &discovered.info.interfaces {
                println!("Interface {}", iface.interface_number);
                println!(
                    "  Usage Page: {}",
                    iface
                        .usage_page
                        .map(|u| format!("0x{:04x}", u))
                        .unwrap_or_else(|| "Unknown".into())
                );
                println!(
                    "  Usage: {}",
                    iface
                        .usage
                        .map(|u| format!("0x{:04x}", u))
                        .unwrap_or_else(|| "Unknown".into())
                );
                if !iface.report_ids.is_empty() {
                    println!("  Report IDs: {:?}", iface.report_ids);
                }
                if let Some(node) = &iface.hidraw_path {
                    println!("  Device Node: {}", node);
                }
                println!();
            }

            // Display driver capabilities
            let driver_device = DeviceRegistry::create_device(discovered);
            println!("Declared Capabilities:");
            for cap in driver_device.capabilities() {
                let status = driver_device.support_status(*cap);
                println!("  - {:<18} : [{}]", cap, status);
            }

            Ok(())
        }

        Commands::Probe { output } => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            let report = DeviceProber::probe_device(&discovered)?;

            if let Some(ref path) = output {
                DeviceProber::save_to_file(&report, path)?;
                println!("Probe report successfully saved to {}", path.display());
            }

            if cli.json || output.is_none() {
                println!("{}", serde_json::to_string_pretty(&report)?);
            }

            Ok(())
        }

        Commands::Rgb { action } => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            let mut driver = DeviceRegistry::create_device(discovered);

            match action {
                RgbCommands::Static { color } => {
                    let rgb: RgbColor = color.parse()?;
                    let config = LightingConfig {
                        mode: LightingMode::Static,
                        color: rgb,
                        brightness: 100,
                        speed: 0,
                        direction: None,
                    };
                    driver.set_lighting(&config)?;
                    println!(
                        "Set static RGB color to {rgb} on {}",
                        driver.info().product_name
                    );
                }
                RgbCommands::Brightness { level } => {
                    let config = LightingConfig {
                        brightness: level.min(100),
                        ..Default::default()
                    };
                    driver.set_lighting(&config)?;
                    println!(
                        "Set brightness to {}% on {}",
                        config.brightness,
                        driver.info().product_name
                    );
                }
                RgbCommands::Mode {
                    mode,
                    color,
                    brightness,
                    speed,
                } => {
                    let lmode: LightingMode = mode.parse()?;
                    let rgb: RgbColor = color.parse()?;
                    let config = LightingConfig {
                        mode: lmode,
                        color: rgb,
                        brightness: brightness.min(100),
                        speed: speed.min(100),
                        direction: None,
                    };
                    driver.set_lighting(&config)?;
                    println!(
                        "Set lighting mode to '{}' (color: {}, brightness: {}%, speed: {}%) on {}",
                        lmode,
                        rgb,
                        config.brightness,
                        config.speed,
                        driver.info().product_name
                    );
                }
            }
            Ok(())
        }

        Commands::Actuation { action } => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            let mut driver = DeviceRegistry::create_device(discovered);

            match action {
                ActuationCommands::Get => {
                    let point = driver.get_actuation()?;
                    println!("Current actuation point: {}", point);
                }
                ActuationCommands::Set { distance_mm } => {
                    let point = ActuationPoint::new(distance_mm)?;
                    driver.set_actuation(point)?;
                    println!("Updated switch actuation point to {}", point);
                }
            }
            Ok(())
        }

        Commands::RapidTrigger { action } => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            let mut driver = DeviceRegistry::create_device(discovered);

            match action {
                RapidTriggerCommands::Status => {
                    let config = driver.get_rapid_trigger()?;
                    println!(
                        "Rapid Trigger: {}",
                        if config.enabled {
                            "Enabled"
                        } else {
                            "Disabled"
                        }
                    );
                    println!(
                        "  Press Sensitivity   : {:.2} mm",
                        config.press_sensitivity_mm
                    );
                    println!(
                        "  Release Sensitivity : {:.2} mm",
                        config.release_sensitivity_mm
                    );
                }
                RapidTriggerCommands::Enable {
                    press_mm,
                    release_mm,
                } => {
                    let config = RapidTriggerConfig::new(true, press_mm, release_mm)?;
                    driver.set_rapid_trigger(&config)?;
                    println!(
                        "Enabled Rapid Trigger (press: {:.2}mm, release: {:.2}mm)",
                        press_mm, release_mm
                    );
                }
                RapidTriggerCommands::Disable => {
                    let config = RapidTriggerConfig {
                        enabled: false,
                        press_sensitivity_mm: 0.2,
                        release_sensitivity_mm: 0.2,
                    };
                    driver.set_rapid_trigger(&config)?;
                    println!("Disabled Rapid Trigger");
                }
            }
            Ok(())
        }

        Commands::PollingRate { action } => {
            let discovered = find_target_device(&cli.vid, &cli.pid)?;
            let mut driver = DeviceRegistry::create_device(discovered);

            match action {
                PollingRateCommands::Get => {
                    let rate = driver.get_polling_rate()?;
                    println!("Current polling rate: {}", rate);
                }
                PollingRateCommands::Set { rate } => {
                    let polling_rate: PollingRate = rate.parse()?;
                    driver.set_polling_rate(polling_rate)?;
                    println!("Updated USB polling rate to {}", polling_rate);
                }
            }
            Ok(())
        }

        Commands::Permissions => {
            println!("OpenDragon Linux Permission & Udev Diagnostic");
            println!("{:-<60}", "");

            #[cfg(target_os = "linux")]
            {
                let mut accessible = 0;
                let mut total = 0;

                if let Ok(entries) = std::fs::read_dir("/dev") {
                    for entry in entries.flatten() {
                        let fname = entry.file_name().to_string_lossy().to_string();
                        if fname.starts_with("hidraw") {
                            total += 1;
                            let p = entry.path();
                            let can_rw = std::fs::OpenOptions::new()
                                .read(true)
                                .write(true)
                                .open(&p)
                                .is_ok();
                            let can_ro = std::fs::OpenOptions::new().read(true).open(&p).is_ok();
                            if can_rw {
                                accessible += 1;
                                println!("  {} : Read/Write OK", p.display());
                            } else if can_ro {
                                println!("  {} : Read-Only (Cannot send commands)", p.display());
                            } else {
                                println!("  {} : Permission Denied (Root only)", p.display());
                            }
                        }
                    }
                }

                println!(
                    "\nHIDRaw Access: {}/{} nodes accessible without root",
                    accessible, total
                );
                if accessible < total {
                    println!("\nTo grant unprivileged access to Redragon devices, install the udev rules:");
                    println!("  sudo cp udev/70-opendragon.rules /etc/udev/rules.d/");
                    println!("  sudo udevadm control --reload-rules && sudo udevadm trigger");
                    println!("  sudo usermod -a -G plugdev $USER  (if using group based access)");
                } else {
                    println!("\nAll /dev/hidraw nodes are accessible by current user!");
                }
            }

            #[cfg(not(target_os = "linux"))]
            {
                println!("Non-Linux OS: permissions are managed by OS driver subsystems.");
            }

            Ok(())
        }
    }
}
