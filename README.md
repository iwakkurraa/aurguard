# aurguard (aurg)

### Scan a package with `aurg scan <package>`
Example:
aurg scan firefox

### Inspect dependencies after scanning a package with `aurg inspect <dependency>`
Example: 
aurg inspect bash 
> to inspect a dependency it must be listed in the dependencies of a package scanned with `aurg scan` or `aurg inspect`

## Features 
- Scan packages before installation
- Inspect package details, dependencies, and origins
- Identify: 
    - Official Arch packages
    - AUR packages 
    - Unknown packages
- Recursive dependency inspection
- PKGBUILD security checks (WIP)

## Security Checks (WIP)
- Checksums
- External downloads
- Remote script execution
- Service modifications
- System file modifications
- Potentially dangerous commands

## Installation 
### Requirements 
Install dependencies:
```bash 
sudo pacman -S rust curl
```

