## Raspberry Pico Example Templates

This is just a simple template for setting up a project using [embassy_rp](https://github.com/embassy-rs/embassy/tree/8803128707b8bd9fc9dcea392a62dfd42aa822d2/embassy-rp). This currently pulls all dependencies from the embassy repo because I found some of the examples not working with the latest versions of the dependencies on crates.io. Currently pulling commit `8803128707b8bd9fc9dcea392a62dfd42aa822d2` of the embassy repo. Also will notice the `cargo.toml` has everything including the kitchen sink. Trim what you don't need, this is mostly for beginners(me) to get started with.

## This project now has 3 example branches
- [Barebones Pico W blinky Example on the branch main](https://github.com/fatfingers23/raspberry-pico-w-embassy-template)
- [A Pico non w blinky example on the branch pico_non_w](https://github.com/fatfingers23/raspberry-pico-w-embassy-template/tree/pico_non_w)
- [A simple http server example on the branch pico_w_webserver](https://github.com/fatfingers23/raspberry-pico-w-embassy-template/tree/pico_w_webserver)

## Setup

You can use my getting started article to load these examples found [here](https://baileytownsend.dev/articles/getting-started-with-rust-an-rp2040)

Feel free to leave a issue though if you would like help setting up.

## How do I do xyz?

Check the the [embassy_rp examples](https://github.com/embassy-rs/embassy/tree/8803128707b8bd9fc9dcea392a62dfd42aa822d2/examples/rp). Should ideally be able to take any of those and run it inside of this template, this is what it is based off of.
