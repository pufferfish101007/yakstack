import argparse
import subprocess
import glob


def main():
    parser = argparse.ArgumentParser(prog="Yak Stack build script")
    parser.add_argument(
        "-d", "--debug", action="store_true", help="debug build (default)"
    )
    parser.add_argument("-r", "--release", action="store_true", help="release build")
    parser.add_argument(
        "-i",
        "--images",
        action=argparse.BooleanOptionalAction,
        help="carry out SVG->PNG image conversions",
        default=True,
    )
    args = parser.parse_args()

    if (args.debug and args.release) or ((not args.debug) and (not args.release)):
        print("error: must specify exactly one of --debug or --release")
        parser.print_help()
        exit(1)

    release = args.release
    images = args.images

    if images:
        print("converting SVGs to PNGs")
        for svg in glob.glob("assets/**/*.svg", recursive=True):
            subprocess.run(["inkscape", "--export-overwrite", "--export-type=png", svg])

    rust_toolchains = subprocess.run(
        ["rustup", "toolchain", "list"], stdout=subprocess.PIPE, check=True, text=True
    ).stdout
    nightly = "nightly" in rust_toolchains

    if release:
        print("building in release mode")
        if nightly:
            print("using nightly rustc")
            subprocess.run(
                [
                    "cargo",
                    "+nightly",
                    "build",
                    "-Zprofile-hint-mostly-unused",
                    "--release",
                ],
                check=True,
            )
        else:
            subprocess.run(["cargo", "build", "--release"], check=True)
        print("release mode does not build the full project yet - use godot to export")
    else:
        print("building in debug mode")
        if nightly:
            print("using nightly rustc")
            subprocess.run(
                ["cargo", "+nightly", "build", "-Zprofile-hint-mostly-unused"],
                check=True,
            )
        else:
            subprocess.run(["cargo", "build"], check=True)
        print("finished debug build - now run in godot")

    print("finished!")


if __name__ == "__main__":
    main()
