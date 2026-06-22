# KnotFlow (KnF)


## Project Status

* **Status:** Proof of Concept (PoC).
* **Purpose:** Validation of physical syntax via computational simulation in Rust.
* **License:** MIT.
* **Author:** Froylan Béla Garduño Horváth

<a href="https://doi.org/10.5281/zenodo.20755402"><img src="https://zenodo.org/badge/1273896155.svg" alt="DOI"></a>


## Abstract

KnotFlow (KnF) is a phase-state computing architecture designed to process data streams via a 2-bit core. The system operates as a continuous-flow Lambda Machine, where functional evaluation occurs in a distributed manner. The architecture is axiomatically closed: functionality emerges exclusively from the interaction between a **Gödelian Bit** (internal state) and a **Stochastic Bit** (input stream).

## Architecture

### 1. Technical Specifications

The system minimizes the action $S$ to achieve syntactic stability:

$$S(\phi, \aleph_\tau) = \int_{\Omega} (\nabla\phi \cdot \nabla\phi - \lambda H(\phi, \aleph_\tau)) d\Omega$$

-   **$\nabla\phi \cdot \nabla\phi$:** Phase gradient (XOR discrepancy).
    
-   **$H(\phi, \aleph_\tau)$:** Stress transduction operator ($\tau$).
    
-   **$\delta S = 0$:** Convergence criterion (resonance).
    

### 2. 2-Bit Core and Gear Function ($\mathcal{G}$)

The logical transfer is dictated by the accumulated stress $\tau$, normalized by the input queue depth $Q$: $\tau = \min(\text{len}(Q), 3)$.

**Function $\mathcal{G}(\tau)$:**

| Pressure ($\tau$) | Logical Operator | Gear State |
| --- | --- | --- |
| $\tau < 1$ | $\phi \land \psi$ | Gear 1: Resonance |
| $1 \le \tau < 2$ | $\phi \oplus \psi$ | Gear 2: Analysis |
| $\tau \ge 2$ | $\neg(\phi \oplus \psi)$ | Gear 3: Incompleteness |
    

## Implementation

Rust

```
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

```

    

_Note: This system is axiomatically closed. Any external implementation is classified as an extension, preserving the integrity of the core protocol._

## Contribution and Legal

This project is provided "as is" under the MIT License. As a PoC, the software is provided without warranty. Contributions are welcome to improve the robustness of the transduction process.






# Resumen

KnotFlow (KnF) es una arquitectura de computación basada en estados de fase diseñada para procesar flujos de datos mediante un núcleo de 2 bits. El sistema opera como una Máquina Lambda de flujo continuo, donde la evaluación de funciones ocurre de forma distribuida. La arquitectura es axiomáticamente cerrada: la funcionalidad emerge exclusivamente de la interacción entre un **Bit Gödeliano** (estado interno) y un **Bit Estocástico** (flujo de entrada).

## Arquitectura

### 1. Especificaciones Técnicas

El sistema minimiza la acción $S$ para alcanzar la estabilidad sintáctica:

$$S(\phi, \aleph_\tau) = \int_{\Omega} (\nabla\phi \cdot \nabla\phi - \lambda H(\phi, \aleph_\tau)) d\Omega$$

Donde:

-   $\nabla\phi \cdot \nabla\phi$: Gradiente de fase (discrepancia XOR).
    
-   $H(\phi, \aleph_\tau)$: Operador de transducción de tensiones ($\tau$).
    
-   $\delta S = 0$: Criterio de convergencia (resonancia).
    

### 2. Núcleo de 2 bits y función de marcha ($\mathcal{G}$)

La transferencia lógica está dictada por el esfuerzo acumulado $\tau$, normalizado por la profundidad de la cola de entrada $Q$: $\tau = \min(\text{len}(Q), 3)$.

**Función $\mathcal{G}(\tau)$:**

| Presión ($\tau$) | Operador Lógico | Estado de Marcha |
| --- | --- | --- |
| $\tau < 1$ | $\phi \land \psi$ | Marcha 1: Resonancia |
| $1 \le \tau < 2$ | $\phi \oplus \psi$ | Marcha 2: Análisis |
| $\tau \ge 2$ | $\neg(\phi \oplus \psi)$ | Marcha 3: Incompletitud |

## Implementación

Rust

```
struct Nodo {
    g_bit: u8,
    tau: u8,
}

impl Nodo {
    fn procesar_flujo(&mut self, e_bit: u8, queue_depth: usize) -> Option<u8> {
        let esfuerzo = self.g_bit ^ e_bit;

        if esfuerzo == 0 {
            if self.tau > 0 { self.tau -= 1; }
            None
        } else {
            self.tau = (queue_depth as u8).min(3);
            
            let resultado = match self.tau {
                1 => self.g_bit & e_bit,
                2 => self.g_bit ^ e_bit,
                _ => !(self.g_bit ^ e_bit),
            };

            if self.tau >= 3 {
                self.tau = 0;
                return Some((resultado & 1) ^ 1);
            }
            None
        }
    }
}

```
## Contribución y Aspectos Legales

Este proyecto se entrega "tal cual" (as is) bajo licencia MIT. Al tratarse de una Prueba de Concepto (PoC), el software se proporciona sin garantía de ningún tipo. Las contribuciones son bienvenidas para mejorar la robustez del proceso de transducción.
