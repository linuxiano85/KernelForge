# KernelForge: L'ottimizzatore definitivo del kernel Linux

## Missione
Dominare il gaming su Linux, superando le prestazioni di Windows.

## Caratteristiche
- **Selezione dinamica delle versioni del kernel**: Scopri e seleziona automaticamente le versioni del kernel da kernel.org
- Selezione dei moduli guidata dall'IA
- Ottimizzazioni ispirate a CachyOS
- Sistema di sicurezza a 5 livelli

## Focalizzazione sul gaming
- FUTEX2
- Scheduler BORE
- Patch RT
- Bassa latenza in tutto

## Nuova funzionalità: Catalogo dinamico delle versioni del kernel

KernelForge ora include un sistema di scoperta dinamica delle versioni del kernel che:

- ✅ Scarica automaticamente le versioni disponibili da kernel.org
- 📦 Supporta tutte le versioni dalla 6.6.x alla 6.17.x e oltre
- 💾 Cache intelligente con TTL di 24 ore
- 🔄 Fallback offline con versioni predefinite
- 🏷️ Metadati completi: canale (stable/mainline/longterm), data di rilascio, stato EOL

### Utilizzo della API

```rust
use kernelforge::version_catalog;

// Lista versioni disponibili (usa cache se valida)
let versions = version_catalog::list_available_versions(false).await?;

// Forza aggiornamento (bypassa cache)
let versions = version_catalog::list_available_versions(true).await?;

// Accedi ai metadati delle versioni
for version in versions {
    println!("Versione: {} [{}]", version.version, version.channel);
}
```

Vedi [VERSION_CATALOG.md](src-tauri/VERSION_CATALOG.md) per documentazione completa.

## Tabella di confronto
| Sistema Operativo | Prestazioni  |
|-------------------|--------------|
| Windows           | ...          |
| CachyOS          | ...          |
| Kernel generico   | ...          |

## Schermate
*Placeholder per schermate*

## Istruzioni di installazione e build

### Prerequisiti
- Rust 1.70 o superiore
- Cargo

### Build
```bash
cd src-tauri
cargo build --release
```

### Test
```bash
cd src-tauri
cargo test
```

### Esegui esempio
```bash
cd src-tauri
cargo run --example list_versions
```

## Crediti
Ispirato da CachyOS, Xanmod, Clear Linux.