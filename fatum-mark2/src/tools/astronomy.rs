/// Calculates the Solar Term (0-24) based on the Sun's ecliptic longitude.
///
/// This uses a simplified astronomical algorithm suitable for Feng Shui purposes.
/// It aligns with the 24 Jie Qi (Solar Terms) used in the Chinese Calendar.
///
/// Reference: "Astronomical Algorithms" by Jean Meeus.
///
/// Returns:
/// - Solar Term Index (0-23):
///   0: Vernal Equinox (Chunfen) - Longitude 0
///   1: Pure Brightness (Qingming) - 15
///   ...
///   23: Insects Awaken (Jingzhe) - 345
pub fn get_solar_term(year: i32, month: u32, day: u32) -> u32 {
    let jd = julian_day(year, month, day);
    let long = sun_longitude(jd);
    // Solar terms occur every 15 degrees along the ecliptic.
    let term = (long / 15.0).floor() as u32;
    term % 24
}

/// Converts a Gregorian date to Julian Day Number (JDN).
///
/// Used as the time basis for astronomical calculations.
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

/// Calculates the Sun's Apparent Longitude.
///
/// Simplified algorithm (Low Precision) but sufficient for determining the day of a Solar Term.
fn sun_longitude(jd: f64) -> f64 {
    let d = jd - 2451545.0; // Days since J2000.0
    let g = (357.529 + 0.98560028 * d) % 360.0; // Mean Anomaly
    let q = (280.459 + 0.98564736 * d) % 360.0; // Mean Longitude
    // Equation of Center
    let l = q + 1.915 * g.to_radians().sin() + 0.020 * (2.0 * g).to_radians().sin();
    (l + 360.0) % 360.0
}
