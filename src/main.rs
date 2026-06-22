struct Nodo {
    g_bit: u8,
    tau: u8,
}

impl Nodo {
    fn procesar_flujo(&mut self, e_bit: u8, queue_depth: usize) -> Option<u8> {
        let esfuerzo = self.g_bit ^ e_bit;

        if esfuerzo == 0 {
            // Resonance: Stress dissipation
            if self.tau > 0 { self.tau -= 1; }
            None
        } else {
            // Automatic gear adjustment based on queue pressure
            self.tau = (queue_depth as u8).min(3);
            
            let resultado = match self.tau {
                1 => self.g_bit & e_bit,        // Gear 1: AND
                2 => self.g_bit ^ e_bit,        // Gear 2: XOR
                _ => !(self.g_bit ^ e_bit),     // Gear 3: XNOR
            };

            // Network broadcast upon reaching the Poincaré limit
            if self.tau >= 3 {
                self.tau = 0;
                return Some((resultado & 1) ^ 1);
            }
            None
        }
    }
}
