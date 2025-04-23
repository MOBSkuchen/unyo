use lazy_static::lazy_static;
use geolocation::Locator;
use isahc::ReadResponseExt;
use serde_json::Value;
use crate::errors::{UnyoError, UnyoResult};
use crate::ui_renderer::USize;
use crate::weather_widget::{time_with_hour_offset, WeatherWidget};

lazy_static! {
    pub static ref LOCATION: Locator = {
        get_location().expect("Failed to get geo location")
    };
}

fn get_location() -> Option<Locator> {
    for a in get_if_addrs::get_if_addrs().unwrap() {
        if a.ip().is_ipv6() && a.name == "wlan0" {
            return Some(geolocation::find(a.ip().to_string().as_str()).expect("Failed to get GEOLOC"));
        }
    }
    None
}

fn api_req(uri: String) -> UnyoResult<Value> {
    let mut local_data_response = isahc::get(&uri).map_err(|e| {UnyoError::ApiReq(e.to_string(), uri.clone())})?;
    let local_data = local_data_response.text().unwrap();
    serde_json::from_str(&local_data).map_err(|e| {UnyoError::ApiReqFmt(e.to_string(), uri)})
}

pub fn make_api_request() -> Value {
    let (lat, long) = (&LOCATION.latitude, &LOCATION.longitude);
    let uri = format!("https://api.open-meteo.com/v1/forecast?latitude={lat}&longitude={long}&daily=sunshine_duration,temperature_2m_max,temperature_2m_min,uv_index_max,temperature_2m_mean,rain_sum&hourly=temperature_2m,cloud_cover,rain&current=temperature_2m,rain,cloud_cover,is_day&timezone=auto&forecast_hours=24");
    api_req(uri).unwrap()
}

pub struct WeatherInfo {
    pub city: String,
    pub is_day: bool,
    // Temp, Rain, Cloud-coverage (percentage)
    pub current: (f64, f64, i64),
    // Temp (mean), UV, Rain (sum), Sunshine-duration (percentage)
    pub daily: [(f64, f64, f64, f64); 7],
    // Temp, Rain, Cloud-coverage (percentage)
    pub hourly: [(f64, f64, i64, String); 24]
}

impl WeatherInfo {
    pub fn from_json(value: Value) -> Self {
        let head = value.as_object().unwrap();
        let mut city = LOCATION.city.clone();
        city.remove(0);
        city.remove(city.len()-1);
        let mut is_day = false;

        let current = {
            let current = head.get("current").unwrap().as_object().unwrap();
            let rain = current.get("rain").unwrap().as_f64().unwrap();
            let temp = current.get("temperature_2m").unwrap().as_f64().unwrap();
            let cloud_cover = current.get("cloud_cover").unwrap().as_i64().unwrap();
            is_day = current.get("is_day").unwrap().as_i64().unwrap() == 1;
            (temp, rain, cloud_cover)
        };

        let hourly: [(f64, f64, i64, String); 24] = {
            let hourly = head.get("hourly").unwrap().as_object().unwrap();

            let rain = hourly.get("rain").unwrap().as_array().unwrap();
            let temp = hourly.get("temperature_2m").unwrap().as_array().unwrap();
            let cloud_cover = hourly.get("cloud_cover").unwrap().as_array().unwrap();

            core::array::from_fn(|i| (
                temp[i].as_f64().unwrap(),
                rain[i].as_f64().unwrap(),
                cloud_cover[i].as_i64().unwrap(),
                time_with_hour_offset(i as i64)
            ))
        };

        let daily: [(f64, f64, f64, f64); 7] = {
            let daily = head.get("daily").unwrap().as_object().unwrap();
            
            let rain_sum = daily.get("rain_sum").unwrap().as_array().unwrap();
            let temp_mean = daily.get("temperature_2m_mean").unwrap().as_array().unwrap();
            let uv_max = daily.get("uv_index_max").unwrap().as_array().unwrap();
            let sunshine_duration = daily.get("sunshine_duration").unwrap().as_array().unwrap();

            core::array::from_fn(|i| (
                temp_mean[i].as_f64().unwrap(),
                uv_max[i].as_f64().unwrap(),
                rain_sum[i].as_f64().unwrap(),
                sunshine_duration[i].as_f64().unwrap(),
            ))
        };
        
        Self {current, daily, hourly, city, is_day}
    }
    
    pub fn auto_construct_widget(w_size: &USize) -> WeatherWidget {
        let json = make_api_request();
        WeatherWidget::new(Self::from_json(json), w_size)
    }
}