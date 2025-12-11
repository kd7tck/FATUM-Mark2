# FATUM-MARK2

**Quantum-Powered Metaphysical Engine**

FATUM-MARK2 is a Rust-based backend that merges traditional Chinese Metaphysics (Feng Shui, BaZi, Qi Men Dun Jia, I Ching) with true quantum entropy. It utilizes the **University of Colorado Randomness Beacon (CURBy)** to seed high-performance ChaCha20 simulations, searching for statistical anomalies in quantum noise to drive divination and decision-making results.

> **Note:** This system is designed for entertainment and experimental purposes. It explores the intersection of quantum mechanics and ancient divination algorithms.

## Features

### 1. Quantum Entropy Engine
*   **Source:** Fetches true random pulses from the CURBy beacon (`https://random.colorado.edu`).
*   **Simulation:** Seeds a ChaCha20 CSPRNG to run millions of reproducible simulations per request.
*   **Anomaly Detection:** Calculates Z-scores to identify outcomes that deviate significantly from expected probability distributions.

### 2. Traditional Feng Shui (Xuan Kong Flying Stars)
*   **Flying Star Charts:** Generates Annual, Monthly, and Daily charts based on construction period and facing direction.
*   **Replacement Charts (Ti Gua):** Automatically calculates replacement stars when the facing direction aligns with specific "Great Void" lines.
*   **Special Formations:** Detects "Sum of Ten", "Parent String", "Pearl String", and "Seven Star Robbery" patterns.
*   **Period 9 Compliance:** Analyzes charts for compatibility with the current Period 9 (2024-2044) energy cycle.

### 3. Four Pillars of Destiny (BaZi)
*   **Solar Terms:** Uses astronomical algorithms to calculate precise solar terms (Jie Qi) for accurate Month Pillar determination.
*   **Quantum Flux:** Simulates real-time elemental strength variations based on quantum entropy.
*   **Probabilistic Birth:** Simulates alternate "timelines" by adjusting the birth hour based on entropy fluctuations.

### 4. Qi Men Dun Jia (Mystical Doors)
*   **Chai Bu Method:** Implements the Chai Bu method for determining the chart structure.
*   **Full Plate Rotation:** Calculates Earth, Heaven, Door, and Deity plates.
*   **Yin/Yang Dun:** Automatically detects Yin or Yang Dun cycles based on the solar term.

### 5. I Ching Divination
*   **Coin Method Simulation:** Simulates the traditional 3-coin toss method using quantum-seeded RNG.
*   **Hexagram Generation:** Generates the primary hexagram and any changing lines to form the transformed hexagram.
*   **Interpretation:** Provides judgments and image texts for the resulting hexagrams.

### 6. San He & Advanced Water Methods
*   **Double Mountain:** Analyzes the 24 Mountains frame (Water, Wood, Fire, Metal, Earth).
*   **Killings:** Identifies "Yellow Springs" and "Eight Killings" forces based on facing and water exit directions.

## Architecture

*   **Backend:** Rust (Axum, Tokio, Reqwest)
*   **Frontend:** HTML5/CSS3 (Cyberpunk aesthetic), Vanilla JS
*   **Persistence:** SQLite (SQLx) for user history and profiles.
*   **Math:** `rand_chacha` for simulations, `genpdf` for report generation.

## Installation & Usage

### Prerequisites
*   Rust (latest stable)
*   SQLite

### Running the Server
The application runs as a local web server.

```bash
# Clone the repository
git clone https://github.com/your-repo/fatum-mark2.git
cd fatum-mark2

# Run the server
cargo run
```

Once running, open your browser to `http://127.0.0.1:3000`.

### Development
*   **Frontend:** The frontend assets are located in `static/`.
*   **Backend:** Core logic is in `src/tools/` (Feng Shui logic) and `src/engine/` (Simulation).

## License
MIT License
