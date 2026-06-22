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
