use rand::{random_bool, random_range};
use serde_json::{from_str, to_string};
use skju_core::common::{Coord, SensorConfig, SensorData};
use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let sensors = get_sensors().unwrap_or_default();

    if sensors.is_empty() {
        println!("There are no sensors to run");
        return;
    }

    std::thread::scope(|scope| {
        for sensor in sensors {
            scope.spawn(move || {
                if let Err(e) = generate_sensor_data(sensor.id) {
                    eprintln!("{:?}", e);
                }
            });
        }
    });
}

fn get_sensors() -> anyhow::Result<Vec<SensorConfig>> {
    let file_path = verify_path_exists("data/sensors.json")?;
    let mut file_data = String::new();
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open(file_path)?;

    file.read_to_string(&mut file_data)?;

    if file_data.trim().is_empty() {
        file_data = to_string(&get_default_sensors())?;
        file.write_all(file_data.as_bytes())?;
        file.flush()?;
    }

    let sensors: Vec<SensorConfig> = from_str(&file_data)?;

    Ok(sensors)
}

fn generate_sensor_data(sensor_id: u64) -> anyhow::Result<()> {
    let path = format!("data/sensor_{}.txt", sensor_id);
    let file_path = verify_path_exists(&path)?;
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);
    let mut previous_value = 0.0;
    let mut now = SystemTime::now();
    let mut consecutive_spikes_left = 0;

    println!("Writing sensor readings into {}...", file_path.display());

    loop {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();
        let value = generate_random_reading(&mut previous_value, consecutive_spikes_left > 0);
        let reading_json = to_string(&SensorData { timestamp, value })?;
        let next_reading_in = random_range(10..=20);

        if consecutive_spikes_left > 0 {
            consecutive_spikes_left -= 1;
        } else {
            let new_spike = random_bool(0.005);
            consecutive_spikes_left = if new_spike { random_range(4..10) } else { 0 };
        }

        writeln!(writer, "{reading_json}")?;

        if now.elapsed()?.as_millis() > 100 {
            writer.flush()?;
            now = SystemTime::now();
        }

        std::thread::sleep(Duration::from_millis(next_reading_in));
    }
}

fn generate_random_reading(last_value: &mut f64, with_spike: bool) -> f64 {
    let value: f64 = random_range(-0.01..=0.01);
    let spike_dir = if random_bool(0.5) { -1.0 } else { 1.0 };
    let spike: f64 = random_range(1.5..=3.0) * spike_dir;

    *last_value += value - 0.02 * *last_value;
    *last_value = (*last_value).clamp(-1.0, 1.0);

    if with_spike {
        *last_value += spike;
    }

    *last_value
}

fn get_default_sensors() -> Vec<SensorConfig> {
    Vec::from([
        SensorConfig {
            id: 1,
            name: String::from("Sensor Alpha"),
            coord: Coord { x: 12.5, y: 34.8 },
        },
        SensorConfig {
            id: 2,
            name: String::from("Sensor Beta"),
            coord: Coord { x: 18.2, y: 29.1 },
        },
        SensorConfig {
            id: 3,
            name: String::from("Sensor Gamma"),
            coord: Coord { x: 25.7, y: 40.3 },
        },
    ])
}

fn verify_path_exists(path: &str) -> anyhow::Result<&Path> {
    let file_path = Path::new(path);

    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    Ok(file_path)
}
