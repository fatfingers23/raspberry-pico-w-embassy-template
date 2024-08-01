## Raspberry Pico W Embassy Template

This is just a simple template for setting up a project using [embassy_rp](https://github.com/embassy-rs/embassy/tree/2d678d695637ed1023fd80fea482d60a288e4343/embassy-rp). This currently pulls all dependencies from the embassy repo because I found some of the examples not working with the latest versions of the dependencies on crates.io. Currently pulling commit `2b031756c6d705f58de972de48f7300b4fdc673c` of the embassy repo. Also will notice the `cargo.toml` has everything including the kitchen sink. Trim what you don't need, this is mostly for beginners(me) to get started with.

Will notice this is the Wifi Blinky example. I did this so I can include the cyw43 firmware and an example of how to load it onto the pico.

## Setup

Refer to [embassy](https://github.com/embassy-rs/embassy). Feel free to leave a issue though if you would like help setting up.

## How do I do xyz?

Check the the [embassy_rp examples](https://github.com/embassy-rs/embassy/tree/2d678d695637ed1023fd80fea482d60a288e4343/examples/rp). Should ideally be able to take any of those and run it inside of this template, this is what it is based off of.
