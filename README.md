# The project that will make my life easier

## About

This project was originally intended for my own use, but I thought I might as well open-source it in case it's useful to someone else.
It solves a simple problem: I don't wanna spend 20 minutes looking for a free room ever time I don't have class. It queries the ADE API,
retrieves the schedule for every room, and creates a new calendar that shows what rooms are free at any given time.
It exposes a simple web API that re-computes this calendar (in case there were any changes) and outputs every time it is queried.
Therefore, if you host this utility, you can add it to you ADE calendar app by using it as an input URL (see the [ADE app integration](#ade-app-integration) section)

## Building and running

This app is a cargo project.

### Requirements

The app requires `pkg-config` at build time, and `openssl` at runtime.
If you're using the Nix package manager (which is **recommended** as the app has already been packaged for it), these dependencies are already being taken care of.
Otherwise, make sure to have them installed using your package manager of choice before trying to build or run the program.

### Building from Source

If using nix, you have 2 options.

**If using flakes (recommended)**, you have to make sure [ enable both flakes and the nix command ](https://nixos.wiki/wiki/Flakes).
You can then simply run:

```bash
nix build
```

**If you don't want to use flakes**, you can run:

```bash
nix-build
```

You can build the app without nix using:

```bash
cargo build --release
```

### Running

If you use nix, you can simply run:

```bash
nix run
```

Or simply execute the binary in result/bin/ade after the build step is complete.

To execute the program without nix, you can run:

```bash
cargo run
```

> [!WARNING]
> Note that this method *does not apply the optimizations from `cago build --release`*.
> If you want the optimizations, you can run the app by simply executing the result from your build.

The project also includes a Dockerfile and docker-compose.yaml. You can therefore also use:

```bash
docker compose up
```

## ADE app integration

This app is exposed through port `7878` (currently this is hardcoded, however it will later become an argument).
Any request to this port will be answered with the updated calendar. Therefore, if you're hosting this app, you can add the calendar to your ADE app:
- Go to "Add a profile"
- Under "Scan QR code", enter the address of your instance (example: https://ade.example.com)
- Name the profile
- Validate the operation
