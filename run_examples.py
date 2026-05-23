from pathlib import Path
import subprocess
import sys
import os

profile = "debug"

for arg in sys.argv[1:]:
    if arg == "-r":
        profile = "release"
    elif arg != "-d":
        print("Usage: python run_examples.py [-r | -d]")
        sys.exit(1)

binary_name = "smelt.exe" if os.name == "nt" else "smelt"

compiler = Path(f"./target/{profile}/{binary_name}")
examples_dir = Path("./examples")

if profile == "debug" and not compiler.exists():
    print("Debug binary not found, building workspace...")
    subprocess.run(
        ["cargo", "build", "--workspace"],
        check=True
    )

for file in sorted(examples_dir.glob("*.nbl")):
    print(f"Running {file} using {profile} build...")

    subprocess.run(
        [
            str(compiler),
            str(file),
            "--run",
            "--clean",
        ],
        check=True
    )
    