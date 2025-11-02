use anyhow::anyhow;
use serde_json::from_str;
use skju_core::common::{SensorConfig, SensorData, SensorOutput};
use skju_core::filter::MultiPoleExponentialLowPass;
use skju_core::sensor::Sensor;
use skju_core::utils::get_sensors_from_file;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

type FilteredSensor = Sensor<MultiPoleExponentialLowPass>;

fn main() {
    let sensors: Vec<SensorConfig> = get_sensors_from_file("data/sensors.json").unwrap_or_default();
    let sensors: Vec<Arc<Mutex<FilteredSensor>>> = sensors
        .into_iter()
        .map(create_sensor_from_config)
        .map(|sensor| Arc::new(Mutex::new(sensor)))
        .collect();

    if sensors.is_empty() {
        println!("There are no sensors to process");
        return;
    }

    std::thread::scope(move |scope| {
        let stop = Arc::new(AtomicBool::new(false));

        for sensor in &sensors {
            let writer = sensor.clone();
            let stop = stop.clone();

            scope.spawn(move || {
                if let Err(e) = read_sensor_data(writer, stop.clone()) {
                    eprintln!("{:?}", e);
                    stop.store(true, Relaxed);
                }
            });
        }

        if let Err(e) = process_sensor_data(&sensors, stop.clone()) {
            eprintln!("{:?}", e);
            stop.store(true, Relaxed);
        }
    });
}

fn create_sensor_from_config(sensor_config: SensorConfig) -> FilteredSensor {
    let capacity = 100;
    let smoothing = 0.1;
    let number_of_stages = 3;

    Sensor::new(sensor_config.id, &sensor_config.name)
        .coord(sensor_config.coord)
        .filter(MultiPoleExponentialLowPass::new(number_of_stages, smoothing))
        .with_capacity(capacity)
        .build()
        .unwrap_or_else(|e| panic!("[ERR={:?}] Unable to create Sensor, missing args", e))
}

fn read_sensor_data(sensor: Arc<Mutex<FilteredSensor>>, stop: Arc<AtomicBool>) -> anyhow::Result<()> {
    let sensor_entity = sensor.lock().map_err(|_| anyhow!("mutex poisoned"))?;
    let file_path = format!("data/sensor_{}.txt", sensor_entity.id);

    drop(sensor_entity);

    let file = OpenOptions::new().read(true).open(file_path)?;
    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::End(0))?;

    loop {
        if stop.load(Relaxed) {
            break;
        }

        let mut line_data = String::new();
        let bytes = reader.read_line(&mut line_data)?;

        if bytes != 0 {
            let sensor_data: SensorData = from_str(line_data.trim())?;
            let mut sensor_entity = sensor.lock().map_err(|_| anyhow!("mutex poisoned"))?;

            sensor_entity.write(sensor_data.value, sensor_data.timestamp);
        }

        sleep(Duration::from_millis(10));
    }

    Ok(())
}

fn process_sensor_data(sensors: &Vec<Arc<Mutex<FilteredSensor>>>, stop: Arc<AtomicBool>) -> anyhow::Result<()> {
    loop {
        if stop.load(Relaxed) {
            break;
        }

        let mut result = Vec::with_capacity(sensors.len());

        for sensor in sensors {
            let mut data = sensor.lock().map_err(|_| anyhow!("mutex poisoned"))?;
            result.push(data.get_latest());
        }

        let result: Option<Vec<SensorOutput>> = result.into_iter().collect();

        if let Some(data) = result {
            data.iter()
                .for_each(|s| println!("[{}]: {} at {}", s.sensor_name, s.value, s.timestamp));
        }

        sleep(Duration::from_millis(10));
    }

    Ok(())
}
