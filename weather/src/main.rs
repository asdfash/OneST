mod api;
use std::io;

use api::Data;
use reqwest::Error;

#[tokio::main]

//main loop
async fn main() -> Result<(), Error> {
    //api request
    let request_url = format!("https://api.data.gov.sg/v1/environment/2-hour-weather-forecast/");
    let response = reqwest::get(&request_url).await?;
    let users: Data = response.json().await?;
    //get a list of all available areas
    let valid_areas = users.get_all_areas();
    let mut area = String::new();

    println!(
        "Please type an area to see the 2 hour weather forecasts. Valid areas are {:?}",
        valid_areas
    );

    let _ = io::stdin().read_line(&mut area);
    let forecast = users.get_forecasts(area);

    println!(
        "the forecast for {} is {}",
        forecast.area, forecast.forecast
    );
    println!("press enter to close",);
    let _ = io::stdin().read_line(&mut String::new());
    Ok(())
}
