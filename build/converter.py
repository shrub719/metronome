import sys


def read_osu(file):
    name = file.split("/")[-1]
    notes = f"# generated from {name}\n\n"

    with open(file, "r") as f:
        lines = f.readlines()

    i = lines.index("[HitObjects]\n")
    for line in lines[i+1:]:
        parts = line.split(",")
        osu_x = int(parts[0])
        ms = int(parts[2])
        type_flags = int(parts[3])
        ms_end = int(parts[5].split(":")[0])

        if osu_x == 64: x = 0.0
        if osu_x == 192: x = 0.333
        if osu_x == 320: x = 0.666
        if osu_x == 448: x = 1.0

        # 1 (bit index 0) is a tap note
        # 128 (bit index 7) is a hold note
        # the first note in the chart also has bit index 2 on to mark the start of a combo
        # so its flag will be 4 more
        if type_flags == 1 or type_flags == 5:
            notes += f"t {ms} {x}\n"
        elif type_flags == 128 or type_flags == 132:
            notes += f"h {ms} {x} {ms_end}\n"
        else:
            print(f"/!\\ unsupported note type with flags {type_flags}")

    return notes


def write_txt(out, notes):
    with open(out, "w") as f:
        f.write(notes)


def main():
    inpt = sys.argv[1]
    out = sys.argv[2]

    # this is like all the validation you're getting
    if inpt[-4:] != ".osu":
        print("/!\\ not an .osu file")
        return

    notes = read_osu(inpt)
    write_txt(out, notes)


if __name__ == "__main__":
    main()
    
