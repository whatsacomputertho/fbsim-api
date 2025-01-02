# FBSim API

> A Rust-based REST API for simulating american football games, based on the fbsim-core crate

## Overview

A REST API built atop the fbsim-core Rust crate which enables users to simulate american football games.

## Usage

### Running the server

To run the server natively, one can simply clone this repository and `cargo run` / `cargo build`.  Alternatively, to run the server as a container, one can again clone this repository and run the following (here, `podman` can optionally be replaced with `docker`).

```sh
# Build the container image
podman build -f Containerfile . -t fbsim-api:latest

# Run the container
podman run -it -p 8080:8080 fbsim-api:latest
```

### Game simulation

Run the server following the instructions above, and then POST a payload like so:

```json
{
    "home": {
        "name": "Home Team",
        "offense_overall": 75,
        "defense_overall": 66
    },
    "away": {
        "name": "Away Team",
        "offense_overall": 99,
        "defense_overall": 88
    }
}
```

To the `/game/sim` endpoint.  Using cURL, this would look like so:
```sh
curl -X POST -H "Content-Type: application/json" -d "{\"home\":{\"name\":\"Home Team\",\"offense_overall\":75,\"defense_overall\": 66},\"away\":{\"name\":\"Away Team\",\"offense_overall\":99,\"defense_overall\":88}}\" http://127.0.0.1:8080
```
