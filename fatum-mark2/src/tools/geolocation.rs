use crate::engine::SimulationSession;
use geo::{Point, HaversineDestination, HaversineDistance};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
    pub power: f64, // Density score
    pub type_: String, // "Attractor", "Void", "Anomaly"
}

pub struct GeolocationTool {
    session: SimulationSession,
}

impl GeolocationTool {
    pub fn new(session: SimulationSession) -> Self {
        Self { session }
    }

    /// Generates a quantum anomaly point near the center.
    /// `center_lat`: Latitude of user
    /// `center_lon`: Longitude of user
    /// `radius_meters`: Search radius
    /// `points_count`: How many quantum points to simulate
    pub fn generate_location(&self, center_lat: f64, center_lon: f64, radius_meters: f64, points_count: usize) -> GeoPoint {
        let center = Point::new(center_lon, center_lat);
        let mut rng = ChaCha20Rng::from_seed(self.session.seed); // Access seed via getter or public field?
        // Note: SimulationSession seed is currently private. I need to fix that or expose a method to get an RNG.
        // For now, I will assume I can access the seed if I make it public or use a method.
        // Let's modify SimulationSession to be more flexible or duplicate the RNG logic here.
        // Actually, better design: SimulationSession should provide a method to get random coordinates.

        // Temporarily, let's assume I can modify SimulationSession or access the seed.
        // I'll make the seed public in `src/engine/mod.rs` in a subsequent step or just re-implement the RNG here if I pass the seed.
        // But GeolocationTool owns the session.

        let mut points: Vec<Point> = Vec::with_capacity(points_count);

        for _ in 0..points_count {
            // Random bearing 0-360
            let bearing = rng.gen_range(0.0..360.0);
            // Random distance 0-radius
            // SQRT for uniform distribution in a circle
            let distance = rng.gen_range(0.0f64..1.0f64).sqrt() * radius_meters;

            let p = center.haversine_destination(bearing, distance);
            points.push(p);
        }

        // Find clusters (Attractors)
        // Simple algorithm: Divide area into a grid (e.g., 10x10) and find the densest cell.
        // Or pick a random subset of points and count neighbors.

        // Let's use a simplified "Density Scan":
        // 1. Pick X random "probe" points from the generated set.
        // 2. Count neighbors within Y meters (e.g., 50m) for each probe.
        // 3. The probe with the highest count is the Attractor.

        let mut best_point = center;
        let mut max_neighbors = 0;

        // Scan 100 random points as candidates (or all if count is low)
        let candidates_count = if points_count > 500 { 500 } else { points_count };

        for _ in 0..candidates_count {
            let candidate_idx = rng.gen_range(0..points.len());
            let candidate = points[candidate_idx];

            let neighbors = points.iter()
                .filter(|&&p| p.haversine_distance(&candidate) < 50.0) // 50m radius density check
                .count();

            if neighbors > max_neighbors {
                max_neighbors = neighbors;
                best_point = candidate;
            }
        }

        GeoPoint {
            lat: best_point.y(),
            lon: best_point.x(),
            power: max_neighbors as f64,
            type_: "Attractor".to_string(),
        }
    }
}
