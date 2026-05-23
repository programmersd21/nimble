import subprocess
import sys

profile = "debug"

if len(sys.argv) > 1:
    if sys.argv[1] == "-r":
        profile = "release"
    elif sys.argv[1] != "-d":
        print("Usage: python build.py [-r | -d]")
        sys.exit(1)

cmd = ["cargo", "build", "--workspace"]

if profile == "release":
    cmd.append("--release")

print(f"Building workspace with {profile} profile...")
subprocess.run(cmd, check=True)
