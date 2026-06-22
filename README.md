# KnotFlow (KnF)

**Status:** Proof of Concept  
**License:** MIT  
**Author:** Froylan Béla Garduño Horváth  
**DOI:** [10.5281/zenodo.20755402](https://doi.org/10.5281/zenodo.20755402)

## What it is

KnotFlow is a 2-bit computing architecture that processes data streams 
through the interaction of two elements:

- **Gödelian Bit (`g_bit`):** the node's internal state. A precise, 
  repeatable axiom — not a truth, but a stable reference point. 
  Analogous to a scale's tare: deliberately fixed, internally consistent.
- **Stochastic Bit (`e_bit`):** the input stream. The approximation of 
  reality that contrasts against the tare.

The system does not seek truth. It measures the sustained discrepancy 
between its internal axiom and external evidence, and recalibrates only 
when that discrepancy is large enough to force it.

## Core mechanic

Stress (`τ`) accumulates when `g_bit ≠ e_bit`. When stress reaches the 
Poincaré limit (τ = 3), the node broadcasts and recalibrates its tare 
to the value that sustained the pressure. This is not learning in the 
conventional sense — it is **forced axiom update under evidence**.

When `g_bit == e_bit`, stress dissipates gradually. The node tends 
toward silence.
Gear 1 (τ=1): AND  — immediate coherence check

Gear 2 (τ=2): XOR  — isolates the discrepancy

Gear 3 (τ=3): XNOR — incompleteness signal, triggers recalibration

## Implementation

```rust
struct Nodo {
    g_bit: u8, // Tare: precise internal axiom. Range {0,1}.
    tau: u8,   // Accumulated stress. Range [0,3].
}

impl Nodo {
    fn new(g_bit: u8) -> Self {
        Self { g_bit: g_bit & 1, tau: 0 }
    }

    /// Returns Some(result) only on broadcast (tare recalibration).
    /// Returns None during resonance or stress accumulation.
    fn procesar_flujo(&mut self, e_bit: u8) -> Option<u8> {
        let e_bit = e_bit & 1;
        let esfuerzo = self.g_bit ^ e_bit;

        if esfuerzo == 0 {
            self.tau = self.tau.saturating_sub(1);
            None
        } else {
            self.tau = (self.tau + 1).min(3);
            let xor_val = self.g_bit ^ e_bit;

            let resultado = match self.tau {
                1 => self.g_bit & e_bit,
                2 => xor_val,
                _ => (xor_val ^ 1) & 1,
            };

            if self.tau >= 3 {
                self.g_bit = e_bit; // tare adopts the value that forced recalibration
                self.tau = 0;
                Some(resultado)
            } else {
                None
            }
        }
    }
}
```

## Formal properties

- Recalibration is never null: broadcast only occurs when `g_bit ≠ e_bit`,
  so `g_bit` always changes.
- `g_bit` and `e_bit` are always in `{0,1}` by construction.
- `tau` never underflows.
- Pure alternating input (`0,1,0,1...`) never triggers broadcast —
  unsustained discrepancy does not force learning.

## Design boundaries

The core is intentionally minimal. The following are **not bugs** — 
they are deferred to extension layers:

- **Mesh topology:** how nodes connect and forward broadcasts.
- **Overflow handling:** what happens when a node saturates repeatedly
  (jail mechanism, planned for the mesh layer).
- **Formal verification:** behavioral analysis via a 
  dependently-typed layer (Agda, planned).
- **Automation of gear selection:** currently manual, 
  by design, for experimental observation.

Contributions that modify core semantics should open a discussion first.
Contributions to extension layers are welcome directly.

## Architecture (formal)

The system minimizes action S to achieve syntactic stability:

$$S(\phi, \aleph_\tau) = 
\int_{\Omega} (\nabla\phi \cdot \nabla\phi - \lambda H(\phi, \aleph_\tau)) d\Omega$$

- $\nabla\phi \cdot \nabla\phi$: phase gradient (XOR discrepancy)
- $H(\phi, \aleph_\tau)$: stress transduction operator
- $\delta S = 0$: convergence criterion (resonance)
