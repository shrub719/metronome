import subprocess
from pathlib import Path

SRC = Path("mtn-maps")
DST = Path("assets/maps")

for src in SRC.rglob("*.mtn"):
    rel = src.relative_to(SRC)
    out = (DST / rel).with_suffix(".mtb")

    out.parent.mkdir(parents=True, exist_ok=True)
    
    if out.exists() and out.stat().st_mtime >= src.stat().st_mtime:
        continue

    print(f"maps: {src} > {out}")

    subprocess.run(
        ["mtn-poly", "compile", str(src), "-o", str(out)],
        check=True,
    )

print("maps: done!")
