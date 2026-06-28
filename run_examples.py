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

binary_name = "nimble.exe" if os.name == "nt" else "nimble"
compiler = Path(f"./target/{profile}/{binary_name}")

if not compiler.exists():
    print(f"{profile} binary not found, building...")
    subprocess.run(["cargo", "build"] + (["--release"] if profile == "release" else []), check=True)

examples_dir = Path("./examples")
passed = []
failed = []

for file in sorted(examples_dir.glob("*.nbl")):
    print(f"Running {file}...")
    result = subprocess.run(
        [str(compiler), "compile", str(file), "-r"],
        capture_output=True, text=True
    )
    if result.returncode == 0:
        passed.append(file.name)
        out = result.stdout.strip()
        if out:
            print(out)
    else:
        failed.append(file.name)
        print(f"  FAILED: {result.stdout.strip() or result.stderr.strip()}")

print(f"\n{len(passed)} passed, {len(failed)} failed")
if failed:
    print("Failed:", ", ".join(failed))
    sys.exit(1)
