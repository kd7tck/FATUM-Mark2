/// Calculates the Solar Term (0-24) based on the Sun's ecliptic longitude.
/// This is a simplified astronomical calculation.
/// Uses a simplified version of VSOP87 or similar approximation for "Low Precision" but better than fixed dates.
/// Reference: "Astronomical Algorithms" by Jean Meeus.
///
/// Returns:
/// - Solar Term Index (0 = Vernal Equinox, 1 = Qingming, ..., 23 = Jingzhe?)
///   Actually:
///   0: Vernal Equinox (Chunfen) - Longitude 0
///   1: Pure Brightness (Qingming) - 15
///   ...
///   21: Great Cold (Dahan) - 300
///   22: Rain Water (Yushui) - 330
///   23: Insects Awaken (Jingzhe) - 345
pub fn get_solar_term(year: i32, month: u32, day: u32) -> u32 {
    let jd = julian_day(year, month, day);
    let long = sun_longitude(jd);
    let term = (long / 15.0).floor() as u32;
    term % 24
}

fn julian_day(year: i32, month: u32, day: u32) -> f64 {
    let mut y = year;
    let mut m = month as i32;
    if m <= 2 {
        y -= 1;
        m += 12;
    }
    let a = (y as f64 / 100.0).floor();
    let b = 2.0 - a + (a / 4.0).floor();
    (365.25 * (y as f64 + 4716.0)).floor() + (30.6001 * (m as f64 + 1.0)).floor() + day as f64 + b - 1524.5
}

/// Simplified Sun Longitude (deg)
fn sun_longitude(jd: f64) -> f64 {
    let d = jd - 2451545.0;
    let g = (357.529 + 0.98560028 * d) % 360.0;
    let q = (280.459 + 0.98564736 * d) % 360.0;
    let l = q + 1.915 * g.to_radians().sin() + 0.020 * (2.0 * g).to_radians().sin();
    (l + 360.0) % 360.0
}
