# Build System Errors

This page documents all error codes in the **Build System** category.

## Summary

Total codes: 39
- 35 error code(s)
- 4 warning code(s)

## Error Codes

---

### N8001: Configuration parse error

- **Severity:** Error
- **Code:** `N8001`

A configuration file could not be parsed. Check the file format and syntax.

---

### N8002: Configuration missing field

- **Severity:** Error
- **Code:** `N8002`

A configuration file is missing a required field. Add the missing field.

---

### N8003: Configuration invalid value

- **Severity:** Error
- **Code:** `N8003`

A configuration field has an invalid value. Check the expected type and valid range.

---

### N8004: Build target not found

- **Severity:** Error
- **Code:** `N8004`

The specified build target does not exist in the project configuration.

---

### N8005: Build script error

- **Severity:** Error
- **Code:** `N8005`

A build script (pre-build, post-build) exited with a non-zero status.

---

### N8006: Missing build dependency

- **Severity:** Error
- **Code:** `N8006`

A build-time dependency is not available. Install the required build tools.

---

### N8007: Invalid build profile

- **Severity:** Error
- **Code:** `N8007`

The specified build profile (e.g., debug, release) is not valid.

---

### N8008: Invalid manifest

- **Severity:** Error
- **Code:** `N8008`

The project manifest file has an invalid format or content.

---

### N8009: Manifest missing package name

- **Severity:** Error
- **Code:** `N8009`

The package manifest is missing the package name field. Add a name field.

---

### N8010: Manifest missing version

- **Severity:** Error
- **Code:** `N8010`

The package manifest is missing the version field. Add a version string.

---

### N8011: Manifest duplicate entry

- **Severity:** Error
- **Code:** `N8011`

The manifest contains duplicate entries for the same key. Remove the duplicates.

---

### N8012: Manifest invalid dependency

- **Severity:** Error
- **Code:** `N8012`

A dependency specification in the manifest is invalid. Check the dependency format.

---

### N8013: Workspace member not found

- **Severity:** Error
- **Code:** `N8013`

A workspace member specified in the manifest does not exist.

---

### N8014: Workspace duplicate member

- **Severity:** Error
- **Code:** `N8014`

A workspace member appears more than once in the members list.

---

### N8015: Invalid toolchain

- **Severity:** Error
- **Code:** `N8015`

The specified toolchain identifier is invalid. Use a recognized toolchain name.

---

### N8016: Toolchain not installed

- **Severity:** Error
- **Code:** `N8016`

The required toolchain is not installed. Install it using the appropriate toolchain manager.

---

### N8017: Invalid target triple

- **Severity:** Error
- **Code:** `N8017`

The specified target triple (e.g., x86_64-pc-windows-msvc) is not recognized.

---

### N8018: Test failed

- **Severity:** Error
- **Code:** `N8018`

One or more tests failed. Check the test output for details.

---

### N8019: Benchmark failed

- **Severity:** Error
- **Code:** `N8019`

One or more benchmarks failed. Check the benchmark output for details.

---

### N8020: Missing test configuration

- **Severity:** Error
- **Code:** `N8020`

A test suite is missing its configuration file.

---

### N8021: Invalid compiler flag

- **Severity:** Error
- **Code:** `N8021`

An invalid compiler flag was specified. Check the available flags.

---

### N8022: Conflicting compiler flags

- **Severity:** Error
- **Code:** `N8022`

Two or more compiler flags conflict with each other. Remove the conflicting flags.

---

### N8023: Unsupported flag for target

- **Severity:** Error
- **Code:** `N8023`

A compiler flag is not supported for the selected target platform.

---

### N8024: Invalid linker flag

- **Severity:** Error
- **Code:** `N8024`

An invalid linker flag was specified.

---

### N8025: Missing linker

- **Severity:** Error
- **Code:** `N8025`

The system linker was not found. Install the required linker tool.

---

### N8026: Missing assembler

- **Severity:** Error
- **Code:** `N8026`

The system assembler was not found. Install the required assembler.

---

### N8027: Output path not writable

- **Severity:** Error
- **Code:** `N8027`

The specified output path is not writable. Check permissions.

---

### N8028: Cache directory not accessible

- **Severity:** Error
- **Code:** `N8028`

The compiler cache directory is not accessible. Check permissions.

---

### N8029: Concurrent build conflict

- **Severity:** Error
- **Code:** `N8029`

Another build process is using the same output directory. Wait for it to finish or use a different output path.

---

### N8030: Build system internal error

- **Severity:** Error
- **Code:** `N8030`

The build system encountered an internal error. Report this bug to the developers.

---

### N8031: Invalid package name

- **Severity:** Error
- **Code:** `N8031`

The package name does not follow the required naming convention.

---

### N8032: Package name invalid characters

- **Severity:** Error
- **Code:** `N8032`

The package name contains invalid characters. Use only alphanumeric characters, hyphens, and underscores.

---

### N8033: Invalid package version

- **Severity:** Error
- **Code:** `N8033`

The package version string does not follow semantic versioning format.

---

### N8034: License not recognized

- **Severity:** Warning
- **Code:** `N8034`

The specified license identifier is not in the recognized license list. Use a standard SPDX identifier.

---

### N8035: Missing package license

- **Severity:** Warning
- **Code:** `N8035`

The package manifest does not specify a license. Add a license field.

---

### N8036: Missing package description

- **Severity:** Warning
- **Code:** `N8036`

The package manifest does not include a description. Add a brief description.

---

### N8037: Invalid edition

- **Severity:** Error
- **Code:** `N8037`

The specified language edition is not recognized or not supported.

---

### N8038: Feature flag not recognized

- **Severity:** Warning
- **Code:** `N8038`

A feature flag specified in the configuration is not recognized. Check available features.

---

### N8039: Feature flag conflict

- **Severity:** Error
- **Code:** `N8039`

Two or more feature flags conflict and cannot be enabled simultaneously.

---

