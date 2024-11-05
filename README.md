# Weather server

This is a simple weather server that provides weather information for a given city. It uses
the [OpenWeatherMap API](https://openweathermap.org/api) to get the weather information, and axum framework to create the server.

The server provides the following endpoints:

- `GET /weather/{city}`: Returns the weather information for the given city.

That's the only endpoint.

It also has a super simple frontend that allows you to search for a city and see the weather 
information. The frontend is built with the askama template engine, tailwind for some basic
css, and htmx to make it more interactive
