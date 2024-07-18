use psutil::cpu::CpuPercentCollector;
//use psutil::sensors::TemperatureSensor;
use std::error::Error;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;

pub fn cpu_usage() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let mut collector = CpuPercentCollector::new()?;
    sleep(Duration::from_millis(750));
    let usage = collector.cpu_percent()?.round() as u64;
    let temp = psutil::sensors::temperatures()[0]
            .as_ref()
            .unwrap()
            .current()
            .celsius()
            .round() as f32;
    Ok(Box::new(format!("{}% ({}°C)", usage, temp)))
}
