# PassmanTRS
## PassmanTRS is a password manager that uses [Tauri](https://tauri.app/) and [Rust](https://www.rust-lang.org/). It is currently in development and not ready for use.
## Features
- [x] basic password storage
- [x] basic password retrieval
- [x] password deletion
- [x] password editing
- [x] master password encrypted in bcrypt
- [x] password encrypted in AES-256-GCM
- [x] copy to clipboard
- [x] Dark & Light theme
- [x] settings
- [x] master password change

## Upcoming Features
- [ ] english translation (currently only german)
- [ ] password generation
- [ ] password strength meter
- [ ] password sharing
- [ ] password expiration
- [ ] password history
- [ ] password import/export
- [ ] easy vault backup
- [ ] search

## News
- ⌈ 07.01.2024: [v0.1.1 released](https://github.com/gamersi/PassmanTRS/releases/tag/app-v0.1.1)
- | 01.01.2024: [v0.1.0 released](https://github.com/gamersi/PassmanTRS/releases/tag/app-v0.1.0)
- | December 2023: Development resumed
- | June 2023: Development paused
- ⌊ May 2023: Development started

## Installation
### Windows
1. Download the latest release from the [releases page](https://github.com/gamersi/PassmanTRS/releases).
2. Install either the .exe or .msi file.
3. Run the application.

### Linux
1. Download the latest release from the [releases page](https://github.com/gamersi/PassmanTRS/releases).
2. Either run the .AppImage file or install the .deb file. You may need to make the .AppImage file executable by running `chmod +x [file].AppImage`.

### Arch Linux
1. You can install the application from the [AUR](https://aur.archlinux.org/packages/passmantrs-bin/) by running `yay -S passmantrs-bin`.

### Mac (Intel only for now)
1. Download the latest release from the [releases page](https://github.com/gamersi/PassmanTRS/releases).
2. Install the .dmg or the .app.tar.gz file by dragging the extracted .app file to your Applications folder.
3. Run the application.

## Building
You can build the application yourself by following these steps:
1. Install [Rust](https://www.rust-lang.org/).
2. Install [Node.js](https://nodejs.org/en/).
3. Clone the repository by running `git clone https://github.com/gamersi/PassmanTRS.git`.
4. Run `cd PassmanTRS`.
5. Run `npm install`.
6. Run `npm run tauri dev` to run the application in development mode or `npm run tauri build` to build the application.

## Contributing
Feel free to contribute to the project by opening a pull request or an issue. If you have any questions, feel free to open an issue.