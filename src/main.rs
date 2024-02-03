use clap::{Parser, ValueEnum};
use log::{debug, error, info, LevelFilter};
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Debug, ValueEnum)]
enum Mode {
    Off,
    Auto,
    Rainbow,
    Breathing,
    ColourCycle,
}

impl From<&Mode> for u8 {
    fn from(value: &Mode) -> Self {
        match value {
            Mode::Off => 0x04,
            Mode::Auto => 0x05,
            Mode::Rainbow => 0x01,
            Mode::Breathing => 0x02,
            Mode::ColourCycle => 0x03,
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Mode::Off => {
                    "off"
                }
                Mode::Auto => {
                    "auto"
                }
                Mode::Rainbow => {
                    "rainbow"
                }
                Mode::Breathing => {
                    "breathing"
                }
                Mode::ColourCycle => {
                    "colourcycle"
                }
            }
        )
    }
}

fn get_default_port() -> String {
    return if cfg!(target_os = "windows") {
        // see https://github.com/serialport/serialport-rs/issues/96
        r#"\\.\COM3"#
    } else {
        // we assume that other than Windows, only Linux is installed on the systems
        "/dev/ttyUSB0"
    }
    .to_string();
}

/// Change the colour of your T9 Plus without pesky drivers.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Configuration {
    /// The device the data is written to.
    #[arg(
    short,
    long,
    default_value_t = get_default_port(),
    )]
    device: String,

    /// Set the LED mode.
    #[arg(
    short,
    long,
    default_value_t = Mode::Off
    )]
    mode: Mode,

    /// Set the brightness of the LEDs.
    #[arg(short, long, default_value_t = 1)]
    brightness: u8,

    /// Set the speed of the animations.
    #[arg(short, long, default_value_t = 1)]
    speed: u8,

    /// Set the speed of the animations.
    #[arg(short = 'r', long, default_value_t = 9600)]
    baud_rate: u32,
}

impl Display for Configuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Mode: {} Brightness: {} Speed: {}",
            self.mode, self.brightness, self.speed
        )
    }
}

fn main() {
    env_logger::builder()
        .format(|buf, rec| {
            writeln!(
                buf,
                "[{}] {}",
                rec.level().as_str().to_ascii_lowercase(),
                rec.args()
            )
        })
        .filter_level(LevelFilter::Info)
        .is_test(true)
        .init();

    let mut config = Configuration::parse();

    // validate the incoming data
    if !Path::new(config.device.as_str()).exists() {
        error!("The path '{}' is not accessible.", config.device);
        exit(1);
    }

    let in_range = |x: &u8| (1..=5).contains(x);

    if !in_range(&config.brightness) {
        error!("The brightness cannot be '{}'.", config.brightness);
        exit(1);
    }

    if !in_range(&config.speed) {
        error!("The speed cannot be '{}'.", config.speed);
        exit(1);
    }

    // the provided numeric values need to be mapped from 1-5 to 5-1
    config.speed = 6 - config.speed;
    config.brightness = 6 - config.brightness;

    // access the port and write the data to it
    let mut port = serialport::new(config.device.as_str(), config.baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    info!("Opened the serial port.");
    debug!("Writing configuration ({})", config);

    let duration = Duration::from_millis(5);

    port.write_all(&[0xfa])
        .expect("Failed to write the start byte.");
    sleep(duration);

    port.write_all(&[u8::from(&config.mode)])
        .expect("Failed to write the start byte.");
    sleep(duration);

    port.write_all(&[config.brightness])
        .expect("Failed to write the brightness byte.");
    sleep(duration);

    port.write_all(&[config.speed])
        .expect("Failed to write the speed byte.");
    sleep(duration);

    port.write_all(&[create_checksum(&config)])
        .expect("Failed to write the checksum byte.");
}

fn create_checksum(config: &Configuration) -> u8 {
    // create a sum of the values in advance so we do not get an overflow
    let sum = u8::from(&config.mode) + config.brightness + config.speed;
    let sum = sum as u16 + 0xfa;

    return (sum & 0x00ff) as u8;
}

// the tests verify the values shown here:
// https://old.reddit.com/r/MiniPCs/comments/18icusg/t9_plus_n100_how_to_control_led/
#[test]
fn rainbow_bright_fast() {
    let config: Configuration = Configuration {
        device: "".to_string(),
        mode: Mode::Rainbow,
        brightness: 1,
        speed: 1,
    };
    println!("Writing configuration ({})", config);

    assert_eq!(0xfd, create_checksum(&config));
}

#[test]
fn breathing_bright_quite_fast() {
    let config: Configuration = Configuration {
        device: "".to_string(),
        mode: Mode::Breathing,
        brightness: 1,
        speed: 2,
    };
    println!("Writing configuration ({})", config);

    assert_eq!(0xff, create_checksum(&config));
}

#[test]
fn breathing_bright_quite_slow() {
    let config: Configuration = Configuration {
        device: "".to_string(),
        mode: Mode::Rainbow,
        brightness: 1,
        speed: 4,
    };
    println!("Writing configuration ({})", config);

    assert_eq!(0x00, create_checksum(&config));
}
