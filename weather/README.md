# OneST

For displaying 2 hour weather data in a location in Singapore based off <https://datagovsg-api.readthedocs.io/en/latest/#datagovsg_api.EnvironmentAPI.weather_forecast>

Upon building, navigate to target and run the executable based off your computer's specific hardware.

## Overview of files

main.rs creates a get request for the api, before formatting into a Json as per data. It then prompts the user for the area, giving suggestions. With a vaild area, it provides the data in the console

api.rs contains the data structure of the json file, as well as the functions to look up and format the data.

Dependencies used are stated in Cargo.toml
