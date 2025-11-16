# Biological Visual System Implementation

This document explains how the biological visual system works and how it's implemented in this project.

## 🧠 Overview of the Visual Pathway

```
Light → Retina → Thalamus → Primary Visual Cortex (V1) → Perception
         ↓         ↓            ↓
       Cones    Ganglion    Orientation
                 Cells       Detection
```

---

## 1️⃣ Photoreceptors: Cone Cells

**File**: `src/cone.rs`

### Biological Background

Cones are photoreceptor cells in the retina responsible for color vision and fine detail detection.

#### Three Types of Cones

| Type | Peak Wavelength | Color Sensitivity | Pigment |
|------|----------------|-------------------|---------|
| **S-cones** | 420 nm | Blue | Short-wavelength opsin |
| **M-cones** | 530 nm | Green | Medium-wavelength opsin |
| **L-cones** | 560 nm | Red | Long-wavelength opsin |

### Anatomy of a Cone

```
┌─────────────────┐
│ Outer Segment   │  Contains photopigments (opsins)
│   - Discs       │  Light absorption happens here
│   - Opsins      │
├─────────────────┤
│ Inner Segment   │  Energy production
│   - Mitochondria│  ATP synthesis
│   - Nucleus     │
└─────────────────┘
```

### The Phototransduction Cascade

This is the biochemical process that converts light into an electrical signal.

#### Step-by-Step Process

```
1. Photon hits opsin
        ↓
2. Opsin changes shape (photoisomerization)
        ↓
3. Activates transducin (G-protein)
        ↓
4. Transducin activates phosphodiesterase (PDE)
        ↓
5. PDE hydrolyzes cGMP → cGMP levels DROP
        ↓
6. cGMP-gated ion channels CLOSE
        ↓
7. Cell membrane HYPERPOLARIZES (-40 mV → -70 mV)
        ↓
8. LESS glutamate released to bipolar cells
```

#### Biological Paradox

**Light INHIBITS the photoreceptor!**

- **In darkness**: Cones are depolarized (-40 mV), releasing glutamate continuously
- **In light**: Cones hyperpolarize (-70 mV), STOP releasing glutamate

This is counterintuitive but allows for efficient signal encoding.

### Implementation

```rust
// src/cone.rs - phototransduction()

// 1. Light intensity based on spectral sensitivity
let sensitivity = self.cone_type.spectral_sensitivity(light.wavelength);
let effective_intensity = light.intensity * sensitivity;

// 2. More light → less cGMP
let target_cgmp = CGMP_DARK_LEVEL - (adapted_intensity / 10.0);

// 3. Less cGMP → channels close → hyperpolarization
let channel_opening = self.cgmp_level / CGMP_DARK_LEVEL;
self.membrane_potential = LIGHT_POTENTIAL + (DARK_POTENTIAL - LIGHT_POTENTIAL) * channel_opening;

// 4. Hyperpolarization → less glutamate
self.glutamate_release = LIGHT_GLUTAMATE_RELEASE 
    + (DARK_GLUTAMATE_RELEASE - LIGHT_GLUTAMATE_RELEASE) * depolarization_factor;
```

### Light Adaptation

Cones adapt to ambient light levels over time:
- **Dark adaptation**: Increases sensitivity in low light (minutes to hours)
- **Light adaptation**: Decreases sensitivity in bright light (seconds to minutes)

```rust
// Adaptation gradually adjusts to sustained light
let adaptation_rate = 0.01;
let target_adaptation = (effective_intensity / 100.0).clamp(0.0, 1.0);
self.adaptation_level += (target_adaptation - self.adaptation_level) * adaptation_rate;
```

---

## 2️⃣ Ganglion Cells: Edge Detection

**File**: `src/ganglion.rs`

### Biological Background

Ganglion cells are the output neurons of the retina. Their axons form the optic nerve that carries visual information to the brain.

**Discovery**: Stephen Kuffler (1953) discovered the center-surround receptive field organization.

### Center-Surround Receptive Fields

Each ganglion cell responds to light in a circular area (receptive field) with two regions:
- **Center**: Small circular area
- **Surround**: Ring-shaped area around the center

### Two Types of Ganglion Cells

#### ON-Center Cells
```
Light in CENTER → Excitation (+)
Light in SURROUND → Inhibition (-)

Response = Center - Surround
```

**Best stimulus**: Small bright spot on dark background

#### OFF-Center Cells
```
Dark in CENTER → Excitation (+)
Dark in SURROUND → Inhibition (-)

Response = Surround - Center
```

**Best stimulus**: Small dark spot on bright background

### Why Center-Surround?

This organization achieves **edge detection** and **contrast enhancement**:

| Stimulus | Center | Surround | Response |
|----------|--------|----------|----------|
| Uniform bright | High | High | **0** (no edge) |
| Uniform dark | Low | Low | **0** (no edge) |
| Edge (bright center) | High | Low | **Strong +** |
| Edge (dark center) | Low | High | **Strong -** |

### Implementation

```rust
// src/ganglion.rs - compute_response()

// Sample pixels in receptive field
for each pixel in receptive field {
    let distance = distance_from_center(pixel);
    
    if distance <= center_radius {
        center_sum += pixel_intensity;
    } else if distance <= surround_radius {
        surround_sum += pixel_intensity;
    }
}

// Center-surround antagonism
let response = match cell_type {
    OnCenter => center_activation - surround_activation,
    OffCenter => surround_activation - center_activation,
};

// Amplified for visualization (×500)
self.output_rate = (response * 500.0).max(0.0);
```

### Biological Function

1. **Edge detection**: Respond strongly to boundaries
2. **Contrast enhancement**: Amplify differences
3. **Motion detection**: Temporal changes activate ganglion cells
4. **Data compression**: ~100M photoreceptors → ~1M ganglion cells

---

## 3️⃣ Primary Visual Cortex (V1): Orientation Detection

**File**: `src/v1_cortex.rs`

### Biological Background

The primary visual cortex (V1, also called striate cortex) is the first cortical area to receive visual information.

**Nobel Prize 1981**: David Hubel and Torsten Wiesel discovered orientation-selective neurons.

### Orientation-Selective Neurons

V1 neurons respond selectively to edges and bars at specific orientations:
- **Horizontal** (0°)
- **Diagonal-right** (45°)
- **Vertical** (90°)
- **Diagonal-left** (135°)

```
    0° ————    45° ╱      90° │     135° ╲
  Horizontal  Diagonal   Vertical  Diagonal
```

### Two Types of V1 Neurons

#### Simple Cells
- Respond to **specific position AND orientation**
- Have elongated receptive fields with ON/OFF subregions
- Like a "bar detector" at a fixed location

```
   ON region
┌───────────┐  ← Responds to bright bar
│           │     in this orientation
│           │     at this position
└───────────┘
```

#### Complex Cells
- Respond to **orientation regardless of position**
- Pool responses from multiple simple cells
- Larger receptive fields
- Motion-sensitive

### Gabor Filters: Mathematical Model

V1 simple cells are well-modeled by **Gabor filters** (sinusoidal waves × Gaussian envelope):

```
G(x,y) = exp(-(x'² + y'²)/(2σ²)) × cos(2πfx' + φ)

where:
  x' = x·cos(θ) + y·sin(θ)    [rotation by θ]
  y' = -x·sin(θ) + y·cos(θ)
  θ = preferred orientation
  f = spatial frequency
  σ = size of receptive field
```

### Implementation

```rust
// src/v1_cortex.rs - compute_response()

let angle = self.preferred_orientation.radians();
let cos_angle = angle.cos();
let sin_angle = angle.sin();

for each pixel in receptive_field {
    // Project position onto preferred orientation axis
    let projected = (dx * cos_angle + dy * sin_angle).abs();
    let perpendicular = (-dx * sin_angle + dy * cos_angle).abs();
    
    // Elongated receptive field (Gabor-like)
    let orientation_weight = if perpendicular < 2.0 {
        (-perpendicular.powi(2) / 2.0).exp()
    } else {
        0.0
    };
    
    response += edge_strength * orientation_weight;
}

// Amplified for visualization (×10)
self.activation = (response / count as f32 * 10.0).max(0.0);
```

### Cortical Organization

#### Columnar Architecture

V1 is organized into **cortical columns**:
- Each column: ~100 neurons
- All neurons in a column: same orientation preference
- Adjacent columns: slightly different orientations
- Rotation through 180° creates "pinwheel" patterns

```
         Column view
              ↓
    ┌───┬───┬───┬───┐
0°  │   │   │   │   │
45° │   │   │   │   │
90° │   │   │   │   │
135°│   │   │   │   │
    └───┴───┴───┴───┘
```

#### Hypercolumns

A **hypercolumn** contains all orientations for one position in visual space:
- Size: ~1 mm² in human V1
- Contains: all orientations (0° to 180°)
- Covers: ~0.1° of visual field

---

## 🔬 Signal Amplification

The biological signals are very small. For visualization and detection, we amplify them:

### 1. Cone Response (×50)

```rust
// src/cone.rs - response_level()
let base_response = (DARK_POTENTIAL - membrane_potential) / (DARK_POTENTIAL - LIGHT_POTENTIAL);
(base_response * 50.0).min(1.0)
```

**Why**: Biological voltage changes are ~30 mV, need to map to 0-1 range for processing.

### 2. Ganglion Cell Response (×500)

```rust
// src/ganglion.rs - compute_response()
self.output_rate = (response * 500.0).max(0.0);
```

**Why**: Center-surround differences are typically 0.1-0.5, amplify to Hz firing rates.

### 3. V1 Neuron Response (×10)

```rust
// src/v1_cortex.rs - compute_response()
self.activation = ((response / count as f32) * 10.0).max(0.0);
```

**Why**: Cortical responses are subtle, amplify for clear orientation detection.

### 4. Diagonal Normalization (/2.25)

```rust
// src/visual_pathway.rs - extract_features()
diagonal_strength /= 2.25;
```

**Why**: 
- We have 2 diagonal orientations (45° and 135°)
- But only 1 horizontal (0°) and 1 vertical (90°)
- Divide by >2 to compensate for this asymmetry

### 5. Decision Threshold (6%)

```rust
// src/visual_pathway.rs - dominant_orientation()
let threshold = 1.06; // 6% advantage for H/V
```

**Why**: Diagonal detectors respond to H/V edges too. Give H/V a small advantage when close.

---

## 📊 Complete Visual Pathway

```
┌─────────────────────────────────────────────────────┐
│                    RETINA                           │
│                                                     │
│  ┌──────────┐                                      │
│  │  CONES   │  • 3 types (S, M, L)                │
│  │  (64×64) │  • Phototransduction cascade        │
│  │          │  • Light → Hyperpolarization        │
│  └────┬─────┘  • Output: Glutamate (inverted)     │
│       │                                             │
│       ↓                                             │
│  ┌───────────┐                                     │
│  │ BIPOLAR   │  • Process local signals            │
│  │  CELLS    │  • ON/OFF types                     │
│  └─────┬─────┘  • Sign-inverting/preserving       │
│        │                                            │
│        ↓                                            │
│  ┌────────────┐                                    │
│  │ GANGLION   │  • Center-surround fields         │
│  │  CELLS     │  • Edge detection                 │
│  │  (64×64)   │  • Output: Firing rate (Hz)       │
│  └──────┬─────┘                                    │
└─────────┼──────────────────────────────────────────┘
          │ Optic Nerve (~1M axons)
          ↓
┌─────────────────────────────────────────────────────┐
│              THALAMUS (LGN)                         │
│  • Relay station                                    │
│  • Attention control                                │
│  • Maintains center-surround organization          │
└──────────────────┬──────────────────────────────────┘
                   │
                   ↓
┌─────────────────────────────────────────────────────┐
│           PRIMARY VISUAL CORTEX (V1)                │
│                                                     │
│  ┌──────────────────────────────────────┐         │
│  │    Orientation Columns                │         │
│  │                                        │         │
│  │  0° (H)    45°     90° (V)    135°   │         │
│  │  ————      ╱       │          ╲      │         │
│  │                                        │         │
│  │  Simple Cells:                        │         │
│  │  • Position-specific                  │         │
│  │  • Orientation-selective              │         │
│  │                                        │         │
│  │  Complex Cells:                       │         │
│  │  • Position-invariant                 │         │
│  │  • Orientation-selective              │         │
│  └──────────────────────────────────────┘         │
│                                                     │
│  Output: Orientation map (64×64×4)                 │
└─────────────────────────────────────────────────────┘
```

---

## 🧪 Testing and Validation

### Test Images

We use synthetic test images to validate the system:

| Image | Expected Detection | Purpose |
|-------|-------------------|---------|
| `test_horizontal.png` | Horizontal edges | Validate H detection |
| `test_vertical.png` | Vertical edges | Validate V detection |
| `test_diagonal.png` | Diagonal edges | Validate D detection |
| `test_checkerboard.png` | Balanced H/V | Complex patterns |
| `test_face.png` | Mixed orientations | Natural-like image |

### Integration Tests

**File**: `tests/visual_system_integration.rs`

Each test validates:
1. Correct orientation detection
2. Signal strengths in expected ranges
3. Dominant orientation matches expected

```rust
#[test]
fn test_horizontal_stripes_detection() {
    let image = load_and_resize_grayscale("images/input/test_horizontal.png", 64, 64)?;
    let mut pathway = VisualPathway::new(64, 64);
    let response = pathway.process_grayscale_image(&image);
    
    // Horizontal should dominate
    assert!(response.features.horizontal_strength > response.features.vertical_strength);
}
```

### Results Summary

```
✅ 53/53 tests passing
  • 44 unit tests (neuron, cone, ganglion, v1)
  • 6 integration tests (visual system)
  • 3 doc tests
```

---

## 📚 Scientific References

### Key Discoveries

1. **Kuffler, S. W. (1953)**
   - "Discharge patterns and functional organization of mammalian retina"
   - *Journal of Neurophysiology*
   - Discovered center-surround receptive fields

2. **Hubel, D. H., & Wiesel, T. N. (1959)**
   - "Receptive fields of single neurones in the cat's striate cortex"
   - *Journal of Physiology*
   - Discovered orientation-selective neurons

3. **Hubel, D. H., & Wiesel, T. N. (1962)**
   - "Receptive fields, binocular interaction and functional architecture in the cat's visual cortex"
   - *Journal of Physiology*
   - Distinguished simple and complex cells

4. **Hubel, D. H., & Wiesel, T. N. (1968)**
   - "Receptive fields and functional architecture of monkey striate cortex"
   - *Journal of Physiology*
   - Described columnar organization

5. **Nobel Prize in Physiology or Medicine (1981)**
   - Awarded to David H. Hubel and Torsten N. Wiesel
   - "For their discoveries concerning information processing in the visual system"

### Computational Models

1. **Marr, D. (1982)**
   - *Vision: A Computational Investigation*
   - Framework for understanding visual processing

2. **Daugman, J. G. (1985)**
   - "Uncertainty relation for resolution in space, spatial frequency, and orientation"
   - Gabor filters as model of V1 simple cells

---

## 🎯 Real-World Application

### Processing a Car Image

```bash
cargo run --example process_image --release -- images/input/car.jpeg
```

**Results**:
```
1️⃣ Photoreceptor Layer (Cones)
   Average cone activation: 58.6%
   Active cones: 3642/4096

2️⃣ Ganglion Cell Layer (Edge Detection)
   Average edge strength: 3.298
   Edge pixels detected: 256/4096

3️⃣ V1 Primary Visual Cortex (Orientation Detection)
   Active V1 regions: 49/4096
   
   Orientation Analysis:
   ├─ Horizontal edges: 475.74  (hood, roof line)
   ├─ Vertical edges:   448.05  (door frames, pillars)
   └─ Diagonal edges:   465.71  (perspective lines)
```

The system successfully detects:
- Horizontal structures (car body lines)
- Vertical structures (door frames)
- Diagonal structures (perspective effects)

---

## 🚀 Usage

### Basic Usage

```rust
use neuron::visual_pathway::VisualPathway;
use neuron::image_utils::load_and_resize_grayscale;

// Load image
let image = load_and_resize_grayscale("image.jpg", 64, 64)?;

// Create visual pathway
let mut pathway = VisualPathway::new(64, 64);

// Process image
let response = pathway.process_grayscale_image(&image);

// Access results
println!("Horizontal: {}", response.features.horizontal_strength);
println!("Vertical: {}", response.features.vertical_strength);
println!("Diagonal: {}", response.features.diagonal_strength);
println!("Dominant: {}", response.features.dominant_orientation());
```

### Generate Test Images

```bash
cargo run --example create_test_images --release
```

### Run Tests

```bash
# All tests
cargo test

# Visual system tests only
cargo test --test visual_system_integration

# With output
cargo test --test visual_system_integration -- --nocapture
```

---

## 🔮 Future Extensions

Potential biological features to add:

1. **Color processing**
   - Color opponent channels (Red-Green, Blue-Yellow)
   - Color constancy

2. **Motion detection**
   - Direction-selective neurons
   - Motion energy models

3. **Higher cortical areas**
   - V2, V4: Complex shapes, textures
   - MT/V5: Motion processing
   - IT: Object recognition

4. **Attention mechanisms**
   - Saliency maps
   - Top-down modulation

5. **Temporal dynamics**
   - Adaptation over time
   - Temporal frequencies

---

## 📝 License

This implementation is for educational purposes, demonstrating how biological visual systems work.

---

*Last updated: November 16, 2025*
