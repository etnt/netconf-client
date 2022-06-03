# netconf-client

Just a Rust experiment, implementing a simple Netconf Client.

Example on how to run what little that works right now:

    cargo run -- --get-config

## Credits

The code is based on the `pipeviewer` project from the great
Video Course: "Hands-On Systems Programming in Rust" by Nathan Stocks.

The transport/ssh code is taken from the netconf-rs project.
