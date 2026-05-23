from pathlib import Path

root_target = Path("target").resolve()

for pattern in ("*.exe", "*.ll"):
    for file in Path(".").rglob(pattern):
        try:
            file.resolve().relative_to(root_target)
            continue
        except ValueError:
            pass

        file.unlink()
        print(f"Deleted: {file}")
        