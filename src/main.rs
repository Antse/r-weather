use structopt::StructOpt;

use reqwest::Error;
use serde::Deserialize;

// take all input arguments
#[derive(StructOpt, Debug)]
#[structopt(name = "request")]
struct Opt {
    #[structopt(short, long)]
    city: String,
    
    #[structopt(short, long)]
    app_key: String,
}

#[derive(Deserialize, Debug)]
struct Wea {
    main: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    #[serde(rename = "name")]
    ville: String,
    weather: Vec<Wea>,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let city_str = opt.city;
    let app_key = opt.app_key;

    let request_url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city_str,
        app_key
    );

    let response = reqwest::get(&request_url).await?.text().await?;

    let w: Weather = serde_json::from_str(&response).unwrap();

    println!(
        "City -> {}\nWeather -> {}",
        w.ville, 
        w.weather[0].description
    );
    Ok(())
}
