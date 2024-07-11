use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    area_metadata: Vec<Metadata>,
    items: Vec<Items>,
    api_info: Info,
}
#[derive(Deserialize, Debug)]
pub struct Metadata {
    name: String,
    label_location: Loc,
}
#[derive(Deserialize, Debug)]
pub struct Loc {
    latitude: f32,
    longitude: f32,
}

#[derive(Deserialize, Debug)]
pub struct Items {
    update_timestamp: String,
    timestamp: String,
    valid_period: Period,
    pub forecasts: Vec<Forecast>,
}
#[derive(Deserialize, Debug)]
pub struct Period {
    start: String,
    end: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Forecast {
    pub area: String,
    pub forecast: String,
}
#[derive(Deserialize, Debug)]
pub struct Info {
    status: String,
}

impl Data {
    //returns only forecast data
    pub fn get_all_forecasts(&self) -> Vec<Forecast> {
        let mut forecasts: Vec<Forecast> = vec![];

        for item in &self.items {
            for forecast in &item.forecasts {
                forecasts.push(forecast.clone())
            }
        }
        return forecasts;
    }

    pub fn get_forecasts(&self, area: String) -> Forecast {
        if area == "".to_string() {
            return Forecast {
                area: "".to_string(),
                forecast: "".to_string(),
            };
        }
        let forecasts = self.get_all_forecasts();
        for forecast in forecasts {
            if clean(forecast.area.clone()) == clean(area.clone()) {
                return forecast;
            }
        }
        return Forecast {
            area,
            forecast: "not found".to_string(),
        };
    }

    pub fn get_all_areas(&self) -> Vec<String> {
        let mut areas: Vec<String> = vec![];

        for item in &self.items {
            for forecast in &item.forecasts {
                areas.push(forecast.area.clone())
            }
        }
        return areas;
    }
}

//helper function to clean up string
fn clean(s: String) -> String {
    
    return s.trim().to_uppercase().replace('"', "");
}
