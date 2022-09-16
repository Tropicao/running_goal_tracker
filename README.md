# Running goal tracker

## Purpose
This tool is a small helper to track progress regarding a yearly running goal :
* define a yearly running goal (for now, update YEARLY_TARGET in main.rs), in kilometers
* log each of your running session in Strava
* run the tool (`cargo run`) to check your progress. Progress is announced assuming you keep an homogeneous training load, distributed all over the year

## Tool configuration
This tools assumes the following points :
* you have a Strava account
* you have configured a Strava application (each free account is eligible to one free API application, with some restrictions). See your settings -> My API Application

When starting the tool for the first time, you will be asked for the application credentials and will be guided to allow your application to access your Strava profile through an OAuth authorization.
The next time, the tool will remember the configuration and will not ask again for credentials

:warning: Credentials retrieved after the first run are currently stored in plain text in a file on you computer ! Make sure to keep this file secret ! If a malicious person retrieve this file, he will be able to read your personal stats on Strava (he will not be able to push data to your Strava profile, since the application is configured by default as "read only")

## Project state
This project has two goals :
* allow to track my yearly running goal progress
* being a Rust sandbox

This project is not meant to be updated on a regular basis and I am not actively working on  it. Feel free to open an issue if this tool could be useful to you but is missing some features, I may take a look at it.