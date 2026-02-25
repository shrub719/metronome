current_target := `rustc -Vv | grep host | awk '{print $2}'`
name := "metronome"

# ===== DEVICE =====

# builds release profile
build: maps
    cargo build --release --bin {{name}} --target=thumbv7em-none-eabihf

# builds dev profile
dev: maps
    cargo build --bin {{name}} --target=thumbv7em-none-eabihf

# loads app to calculator
load: build
    sudo nwlink install-nwa ./target/thumbv7em-none-eabihf/release/{{name}}

# loads dev profile to calculator
dev-load: dev
    sudo nwlink install-nwa ./target/thumbv7em-none-eabihf/debug/{{name}}


# ===== SIMULATOR =====

# builds dev profile for simulator
sim-dev: maps
    cargo build --lib --target={{current_target}}

# run dev profile on simulator
[macos]
sim-dev-run: sim-dev
    ./simulator/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/debug/lib{{name}}.dylib
[linux]
sim-dev-run: sim-dev
    ./simulator/epsilon.bin --nwb ./target/{{current_target}}/debug/lib{{name}}.so


# ===== MAPS =====

maps:
    python3 ./build/maps.py 

# ===== UTILS =====

sim: sim-dev-run

clean:
    cargo clean
