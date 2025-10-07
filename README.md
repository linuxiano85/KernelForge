# KernelForge: L'ottimizzatore definitivo del kernel Linux

## Missione
Dominare il gaming su Linux, superando le prestazioni di Windows.

## Caratteristiche
- Selezione dei moduli guidata dall'IA
- Ottimizzazioni ispirate a CachyOS
- Sistema di sicurezza a 5 livelli
- **Configurazione kernel x86_64-only** per prestazioni massime
- **Rilevamento automatico toolchain** (Clang/LLVM o GCC)
- **LTO ThinLTO** automatico con Clang per build kernel ottimizzate

## Focalizzazione sul gaming
- FUTEX2
- Scheduler BORE
- Patch RT
- Bassa latenza in tutto
- Timer ad alta risoluzione (1000 Hz)
- Preemption completa per responsività desktop

## Architettura e Toolchain

### Architetture supportate
KernelForge si focalizza su **x86_64** per desktop gaming, disabilitando automaticamente:
- ARM/ARM64, MIPS, PowerPC, RISC-V
- S390, IA64, Alpha, M68K, SPARC
- Hardware legacy (ISA, EISA, MCA, floppy, ecc.)

### Selezione automatica del compilatore
1. **Clang/LLVM** (raccomandato): ThinLTO per ottimizzazione kernel
2. **GCC** (fallback): Ottimizzazioni standard per performance

## Utilizzo

### Generare una configurazione kernel

```bash
cd src-tauri
cargo run --example generate_config
```

Questo:
1. Rileva il miglior toolchain disponibile (Clang+LLD o GCC)
2. Genera una configurazione x86_64 ottimizzata per desktop/gaming
3. Applica le categorie di rimozione bloat
4. Mostra il comando make suggerito

### Opzioni di configurazione

```rust
use kernelforge::core::config::KernelConfig;
use kernelforge::core::toolchain::ToolchainDetector;

// Crea configurazione desktop/gaming completa
let config = KernelConfig::desktop_gaming();

// Oppure personalizza
let mut config = KernelConfig::x86_64_baseline();
config.apply_desktop_optimizations();
config.apply_bloat_removal(&["Industrial Hardware Removal"]);

// Genera .config
let config_str = config.emit();

// Rileva toolchain e crea build plan
let detector = ToolchainDetector::new();
let build_plan = detector.create_build_plan()?;
println!("Build con: {}", build_plan.make_command().join(" "));
```

## Requisiti di sistema

- Rust 1.70+
- Per build kernel ottimali:
  - **Raccomandato**: Clang 14+ e LLD
  - **Alternativa**: GCC 11+

### Installazione su Ubuntu/Debian

```bash
# Opzione 1: LLVM/Clang (raccomandato)
sudo apt-get install clang lld

# Opzione 2: GCC (fallback)
sudo apt-get install gcc make
```

## Build e test

```bash
cd src-tauri

# Build
cargo build

# Test
cargo test

# Esegui l'esempio
cargo run --example generate_config
```

## Tabella di confronto
| Sistema Operativo | Prestazioni  |
|-------------------|--------------|
| Windows           | ...          |
| CachyOS          | ...          |
| Kernel generico   | ...          |

## Schermate
*Placeholder per schermate*

## Moduli

### `core::config`
- `KernelConfig`: Builder per generare file .config
- `x86_64_baseline()`: Configurazione base x86_64-only
- `desktop_gaming()`: Configurazione completa desktop/gaming
- `apply_bloat_removal()`: Rimuove hardware non necessario

### `core::toolchain`
- `ToolchainDetector`: Rileva compilatori disponibili
- `BuildPlan`: Piano di build con toolchain e flag
- `Toolchain`: Clang o GCC con info versione
- `LtoConfig`: Configurazione Link Time Optimization

### `core::bloat_removal`
- `BloatRemovalEngine`: Motore per rimuovere moduli non necessari
- Categorie: Architecture, Industrial, Enterprise, Embedded, Legacy, Networking

## Crediti
Ispirato da CachyOS, Xanmod, Clear Linux.