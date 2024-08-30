## Raspberry Pico W Embassy Template

This is just a simple template for setting up a project using [embassy_rp](https://github.com/embassy-rs/embassy/tree/f0a86070512ad739641cee7d9fa39d63f5c8a9f6/embassy-rp). This currently pulls all dependencies from the embassy repo because I found some of the examples not working with the latest versions of the dependencies on crates.io. Currently pulling commit `f0a86070512ad739641cee7d9fa39d63f5c8a9f6` of the embassy repo. Also will notice the `cargo.toml` has everything including the kitchen sink. Trim what you don't need, this is mostly for beginners(me) to get started with.

Will notice this is the Wifi Blinky example. I did this so I can include the cyw43 firmware and an example of how to load it onto the pico.

## Setup

Refer to [embassy](https://github.com/embassy-rs/embassy). Feel free to leave a issue though if you would like help setting up.

## How do I do xyz?

Check the the [embassy_rp examples](https://github.com/embassy-rs/embassy/tree/f0a86070512ad739641cee7d9fa39d63f5c8a9f6/examples/rp). Should ideally be able to take any of those and run it inside of this template, this is what it is based off of.
