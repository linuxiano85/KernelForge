# KernelForge: L'ottimizzatore definitivo del kernel Linux

## Missione
Dominare il gaming su Linux, superando le prestazioni di Windows.

## Caratteristiche
- Selezione dei moduli guidata dall'IA
- Ottimizzazioni ispirate a CachyOS
- Sistema di sicurezza a 5 livelli

## Focalizzazione sul gaming
- FUTEX2
- Scheduler BORE
- Patch RT
- Bassa latenza in tutto

## Tabella di confronto
| Sistema Operativo | Prestazioni  |
|-------------------|--------------|
| Windows           | ...          |
| CachyOS          | ...          |
| Kernel generico   | ...          |

## Schermate
*Placeholder per schermate*

## Istruzioni di installazione e build

### Installazione da pacchetti pre-compilati

Scarica i pacchetti DEB o RPM dalla pagina [releases](https://github.com/linuxiano85/KernelForge/releases).

**Debian/Ubuntu:**
```bash
sudo dpkg -i kernelforge_*_amd64.deb
sudo apt-get install -f
```

**Fedora/RHEL:**
```bash
sudo dnf install kernelforge-*.x86_64.rpm
```

Per istruzioni dettagliate, consulta [docs/packaging.md](docs/packaging.md).

### Build da sorgente

**Requisiti:**
- Rust (1.70+)
- Node.js (16+) - opzionale per frontend personalizzato
- Librerie GTK e WebKit:
  - Debian/Ubuntu: `libwebkit2gtk-4.1-dev libgtk-3-dev`
  - Fedora/RHEL: `webkit2gtk4.1-devel gtk3-devel`

**Compilazione:**
```bash
# Installa dipendenze di sistema
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev  # Debian/Ubuntu
# oppure
sudo dnf install webkit2gtk4.1-devel gtk3-devel  # Fedora

# Compila l'applicazione
cargo install tauri-cli
cargo tauri build

# I pacchetti saranno in src-tauri/target/release/bundle/
```


## Crediti
Ispirato da CachyOS, Xanmod, Clear Linux.