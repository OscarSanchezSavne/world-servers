# WorldServers

**Real-time monitoring of remote servers in a 3D world.**

WorldServers is a desktop application (Rust + Bevy 3D) that visualizes remote Debian/Ubuntu servers as 3D objects with real-time CPU, RAM, disk and network traffic metrics. Each server runs a lightweight agent that captures metrics and outbound connections, and sends them via UDP to the desktop application.

> Open source · developed by **https://savne.net**

---

## Features

- **Server registration** via SSH with support for private keys (with or without passphrase) or password.
- **Interactive 3D world**.
- **Real-time network traffic capture**: inbound and outbound packets between servers, with lines showing each connection.

---

## Getting Started

### Requirements

- Target servers must be **Debian/Ubuntu** with `tcpdump` (`sudo apt install tcpdump`).
- SSH accessible from the machine running the app.

---

## Security

- SSH authentication uses a private key or password.
- Server configuration is stored encrypted in `servers.toml`.

---

## Tests

```bash
cd desktop-app
cargo test
cargo test -- --nocapture
```

---

## Building the AppImage (Linux)

The AppImage is generated inside a **Debian 12 container** to guarantee compatibility with glibc on Debian 12 systems and later.

### 1. Prepare the build container

```bash
cd .devops
docker compose up --build
docker compose run --rm build bash
```

### 2. Inside the container

```bash
# Build release in Debian 12
cd desktop-app
cargo build -p desktop-app --release
cd ..

# Copy the binary to the AppDir
cp desktop-app/target/release/desktop-app desktop-app/AppDir/usr/bin/worldservers

# Set the output and clean libraries so they regenerate from Debian 12
export OUTPUT=/app/desktop-app/AppDir/WorldServers-x86_64.deb12.AppImage
export ARCH=x86_64
export APPIMAGE_EXTRACT_AND_RUN=1

rm -rf desktop-app/AppDir/usr/lib/*

# Generate the AppImage with linuxdeploy
linuxdeploy --appdir desktop-app/AppDir \
  --executable desktop-app/AppDir/usr/bin/worldservers \
  --desktop-file desktop-app/AppDir/usr/share/applications/worldservers.desktop \
  --icon-file desktop-app/AppDir/usr/share/icons/hicolor/256x256/apps/worldservers.png \
  --output appimage
```

The resulting AppImage is placed at `desktop-app/AppDir/WorldServers-x86_64.deb12.AppImage`.

---

## Building the Windows Portable App

The Windows build is exported to `desktop-app/AppDirWindows`.

```powershell
cd desktop-app
cargo build --release

# Recreate the Windows portable directory
Remove-Item AppDirWindows -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force AppDirWindows

# Copy the executable and runtime assets
Copy-Item target\release\desktop-app.exe AppDirWindows\WorldServers.exe
Copy-Item AppDir\usr\share\worldservers\assets AppDirWindows\assets -Recurse

Run the app from:

```powershell
AppDirWindows\WorldServers.exe
```

---

## Persistence

- `.config/servers.toml` — registered servers (encrypted).

---

## License

Open source project under the MIT license. See the [LICENSE](../LICENSE) file.
