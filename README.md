# KernelForge: L'ottimizzatore definitivo del kernel Linux

## Missione
Dominare il gaming su Linux, superando le prestazioni di Windows.

## Caratteristiche
- Selezione dei moduli guidata dall'IA
- Ottimizzazioni ispirate a CachyOS
- Sistema di sicurezza a 5 livelli
- **Supporto multi-versione**: Linux kernel 6.6 LTS e 6.17

## Supporto Kernel

KernelForge supporta ora due versioni del kernel Linux:

### Linux Kernel 6.6 LTS (Predefinito)
- **Stabilità**: Versione LTS (Long Term Support) con aggiornamenti di sicurezza a lungo termine
- **Patch disponibili**: BORE, BBRv3, FUTEX2, PREEMPT_RT
- **Raccomandato per**: Utenti che cercano stabilità e affidabilità a lungo termine

### Linux Kernel 6.17 (Più recente)
- **Novità**: Ultima versione con le funzionalità più recenti
- **Patch disponibili**: BORE (compatibile 6.17), BBRv3 (upstream), FUTEX2 (upstream)
- **Raccomandato per**: Utenti che vogliono le ultime funzionalità e miglioramenti
- **Nota**: Alcune patch di terze parti potrebbero non essere ancora disponibili per questa versione

### Trade-offs: LTS vs Latest

**6.6 LTS:**
- ✅ Supporto a lungo termine
- ✅ Maggiore stabilità
- ✅ Tutte le patch testate e confermate
- ⚠️  Nuove funzionalità arrivano più lentamente

**6.17:**
- ✅ Ultime funzionalità del kernel
- ✅ Miglioramenti delle prestazioni più recenti
- ✅ Molte patch già integrate upstream
- ⚠️  Alcune patch di terze parti potrebbero non essere disponibili
- ⚠️  Supporto a breve termine

## Focalizzazione sul gaming
- FUTEX2 (system call già integrato upstream)
- Scheduler BORE (patch esterna compatibile con entrambe le versioni)
- Patch RT (disponibile per 6.6 LTS)
- Bassa latenza in tutto
- TCP BBRv3 per prestazioni di rete migliorate

## Configurazione baseline

KernelForge genera automaticamente una configurazione di base ottimizzata per x86_64:
- `CONFIG_64BIT`, `CONFIG_X86_64`: Architettura a 64 bit
- `CONFIG_MODULES`: Supporto moduli caricabili
- `CONFIG_PREEMPT`: Preemption completa per bassa latenza
- `CONFIG_HZ_1000`: Timer a 1000Hz per gaming
- Supporto filesystem: ext4, btrfs
- Networking: NET, INET, TCP BBR

## Toolchain

- **Compiler preferito**: Clang/LLVM con lld linker
- **Fallback**: GCC con ld tradizionale
- **LTO**: ThinLTO disponibile come opzione (opt-in)
- Compatibile con entrambe le versioni del kernel

## Tabella di confronto
| Sistema Operativo | Prestazioni  |
|-------------------|-----------------|
| Windows           | ...             |
| CachyOS          | ...             |
| Kernel generico   | ...             |
| **KernelForge**   | **Ottimizzato** |

## Schermate
*Placeholder per schermate*

## Istruzioni di installazione e build
```bash
# Clone del repository
git clone https://github.com/linuxiano85/KernelForge.git
cd KernelForge/src-tauri

# Build del progetto
cargo build --release

# Esecuzione dei test
cargo test
```

## Utilizzo API

```rust
use kernelforge::{KernelVersion, BuildPlan, BuildPlanBuilder};

// Usa la versione predefinita (6.6 LTS)
let default_plan = BuildPlan::new(KernelVersion::default_target());

// Oppure seleziona esplicitamente 6.17
let latest_plan = BuildPlan::new(KernelVersion::V6_17);

// Build plan personalizzato con ThinLTO
let custom_plan = BuildPlanBuilder::new(KernelVersion::V6_6_Lts)
    .enable_thin_lto()
    .build();

// Ottieni patch esterne da applicare
let external_patches = custom_plan.external_patches();
println!("Patch da applicare: {}", external_patches.len());
```

## Crediti
Ispirato da CachyOS, Xanmod, Clear Linux.

## Avvertenza sulle patch di terze parti
Alcune patch di terze parti potrebbero non essere immediatamente disponibili per le versioni più recenti del kernel. KernelForge applica solo patch confermate compatibili con la versione del kernel selezionata. Preferisce sempre le soluzioni upstream quando disponibili.