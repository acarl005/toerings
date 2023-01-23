use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use super::TempHarvest;
use crate::data_harvester::nvidia::NVML_DATA;

pub fn add_nvidia_data(
    temperature_vec: &mut Vec<TempHarvest>,
    temp_type: &TemperatureType,
) -> crate::utils::error::Result<()> {
    if let Ok(nvml) = &*NVML_DATA {
        if let Ok(ngpu) = nvml.device_count() {
            for i in 0..ngpu {
                if let Ok(device) = nvml.device_by_index(i) {
                    if let (Ok(name), Ok(temperature)) =
                        (device.name(), device.temperature(TemperatureSensor::Gpu))
                    {
                        let temperature = temperature as f32;
                        temperature_vec.push(TempHarvest { name, temperature });
                    }
                }
            }
        }
    }

    Ok(())
}
