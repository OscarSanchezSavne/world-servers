# WorldServers

**Monitorización en tiempo real de servidores remotos en un mundo 3D.**

WorldServers es una aplicación de escritorio (Rust + Bevy 3D) que visualiza servidores Debian/Ubuntu remotos como objetos 3D con métricas de CPU, RAM, disco y tráfico de red en tiempo real. Cada servidor ejecuta un agente ligero que captura las métricas y conexiones salientes, y las envía por UDP a la aplicación de escritorio.

> Open source · desarrollado por **https://savne.net**

---

## ✨ Características

- **Registro de servidores** vía SSH con soporte de clave privada (con o sin passphrase) o contraseña.
- **Mundo 3D interactivo** 
- **Captura de tráfico de red** en tiempo real: paquetes entrantes y salientes entre servidores, con líneas que muestran cada conexión.

---

## 🚀 Cómo empezar

### Requisitos

- Los servidores objetivo deben ser **Debian/Ubuntu** con `tcpdump` (`sudo apt install tcpdump`).
- SSH accesible desde la máquina que ejecuta la app.

---

## 🔒 Seguridad

- La autenticación SSH usa clave privada o contraseña.
- La configuración de servidores se guarda cifrada en `servers.toml`.

---

## 🧪 Tests

```bash
cd desktop-app
cargo test
cargo test -- --nocapture 
```

---

## 📦 Generar el AppImage (Linux)

El AppImage se genera dentro de un **contenedor Debian 12** para garantizar compatibilidad con glibc de sistemas Debian 12 y superiores.

### 1. Preparar el contenedor de build

```bash
cd .devops
docker compose up --build
docker compose run --rm build bash
```

### 2. Dentro del contenedor

```bash
# Compilar release en Debian 12
cd desktop-app
cargo build -p desktop-app --release
cd ..

# Copiar binario al AppDir
cp desktop-app/target/release/desktop-app desktop-app/AppDir/usr/bin/worldservers

# Configurar salida y limpiar librerías para que se regeneren desde Debian 12
export OUTPUT=/app/desktop-app/AppDir/WorldServers-x86_64.deb12.AppImage
export ARCH=x86_64
export APPIMAGE_EXTRACT_AND_RUN=1

rm -rf desktop-app/AppDir/usr/lib/*

# Generar el AppImage con linuxdeploy
linuxdeploy --appdir desktop-app/AppDir \
  --executable desktop-app/AppDir/usr/bin/worldservers \
  --desktop-file desktop-app/AppDir/usr/share/applications/worldservers.desktop \
  --icon-file desktop-app/AppDir/usr/share/icons/hicolor/256x256/apps/worldservers.png \
  --output appimage
```

El AppImage resultante queda en `desktop-app/AppDir/WorldServers-x86_64.deb12.AppImage`.

---

## 🗂️ Persistencia

- `.config/servers.toml` — servidores registrados (cifrado).

---

## 📄 Licencia

Proyecto open source bajo licencia MIT. Ver el archivo [LICENSE](../LICENSE).
