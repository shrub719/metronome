<h1 align="center">
    <img src="assets/icons/icon_128.png" alt="Nwagotchi logo" height="100px"> 
    <br />
    metronome
</h1>

**press any key.**

A silent rhythm game for the NumWorks calculator.

**[maps](https://github.com/shrub719/mtn-maps). [polyrhythm](https://github.com/shrub719/mtn-poly).**

## Installation

1. Go to the [latest release](https://github.com/shrub719/metronome/releases/latest)
1. Download `metronome.nwa`
1. Connect to your calculator by USB
1. Go to the [Numworks Installer](https://my.numworks.com/apps) and click **Connect** (make sure your browser has WebUSB capability)
1. Upload `metronome.nwa`
1. Press **Install**


### External map support

External maps in the `.mtb` format can be loaded to the game as a data file.

> **Warning:** as of version 0.3.0, external `.mtb`s are not checked for validity.
An invalid `.mtb` file will cause your program to crash and may erase your save data!

1. Go to the [latest release](https://github.com/shrub719/metronome/releases/latest)
1. Download `metronome_ext.nwa`
1. Connect to your calculator by USB
1. Go to the [Numworks Installer](https://my.numworks.com/apps) and click **Connect** 
   (make sure your browser has WebUSB capability
1. Upload `metronome_ext.nwa`
1. Press **Select a data file**
1. Upload the `.mtb` file of your map
1. Press **Install**

<!-- TODO: simulator -->

## Controls

- Use **D-pad Up/Down** to select a map
- User **D-pad Left/Right** to switch map packs
- Navigate with **Ok**/**Back**
- Exit with **Home**
- **Backspace** to clear all scores (**Ok** confirm/**Back** cancel)

In game, **press any key.**

## Building

### Requirements

- Rust (duh)
- `thumbv7em-none-eabihf` rustup target <!--if building for the calculator-->
- The `just` command runner

### Setup

- `cargo install mtn-poly`
- Clone the [map files](https://github.com/shrub719/mtn-maps) into a subdirectory called `mtn-maps`
- Run `just maps` to compile all the base maps

### Compiling

- `just build`: build the game
- `just load`: load the app to a connected NumWorks calculator
- `just ext-build`: build with external map support
- `just ext-load {file}`: load with the provided `.mtb` file as a data file

<!-- TODO: yk.. simulator.. -->

