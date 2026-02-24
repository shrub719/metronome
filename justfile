current_target := `rustc -Vv | grep host | awk '{print $2}'`

# ===== DEVICE =====

# builds release profile
build:
    cargo build --release --bin metronome --target=thumbv7em-none-eabihf

# builds dev profile
dev:
    cargo build --bin metronome --target=thumbv7em-none-eabihf

# loads app to calculator
load:
    cargo run --release --bin metronome --target=thumbv7em-none-eabihf

# automatically creates .pbj from obj/meshes before loading to calculator
dev-load:
    cargo run --bin metronome --target=thumbv7em-none-eabihf


# ===== SIMULATOR =====

# builds dev profile for simulator
nwb-dev:
    cargo build --lib --target={{current_target}}

# run dev profile on simulator
[macos]
nwb-dev-run: nwb-dev
    ./simulator/epsilon.app/Contents/MacOS/Epsilon --nwb ./target/{{current_target}}/debug/libmetronome.dylib
[linux]
nwb-dev-run: nwb-dev
    ./simulator/epsilon.bin --nwb ./target/{{current_target}}/debug/libmetronome.so


# ===== MAPS =====

convert-osu id:
    python3 ./build/converter.py ./osu/{{id}}.osu ./assets/maps/mtn/osu_pack/{{id}}.mtn

poly id:
    ./poly/mtn-polyrhythm ./assets/maps/mtn/{{id}}.mtn -o ./assets/maps/mtb/{{id}}.mtb


# ===== UTILS =====

ndev: nwb-dev-run

clean:
    cargo clean
