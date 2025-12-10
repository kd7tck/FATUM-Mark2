#[cfg(test)]
mod tests {
    use crate::tools::feng_shui::{calculate_kua_profile, calculate_flying_star_chart};

    #[test]
    fn test_calculate_kua() {
        // Male, 1980 -> Sum 1+9+8+0=18 -> 1+8=9. 11-9=2. Kua 2.
        let k_male = calculate_kua_profile(1980, "M");
        assert_eq!(k_male.number, 2);
        assert_eq!(k_male.group, "West Group");

        // Female, 1985 -> Sum 1+9+8+5=23 -> 5. 4+5=9. Kua 9.
        let k_female = calculate_kua_profile(1985, "F");
        assert_eq!(k_female.number, 9);
        assert_eq!(k_female.group, "East Group");

        // Male 5 -> 2
        let k_m5 = calculate_kua_profile(2004, "M"); // 2+0+0+4=6. 11-6=5. -> 2.
        assert_eq!(k_m5.number, 2);

        // Female 5 -> 8
        let k_f5 = calculate_kua_profile(2011, "F"); // 2+0+1+1=4. 4+4=8. Wait.
        // 2004 F: 6. 4+6=10->1.
        // Let's find a Female 5 case.
        // Sum needs to result in Kua 5. 4+Sum = 5 (Sum=1) or 14 (Sum=10->1).
        // Year 2008: 2+0+0+8=10 -> 1. 4+1=5 -> 8.
        let k_f5_real = calculate_kua_profile(2008, "F");
        assert_eq!(k_f5_real.number, 8);
    }

    #[test]
    fn test_flying_stars_period_8_facing_n2() {
        // Period 8: Construction 2004-2023.
        // Facing N2 (Zi): 360 degrees (0).
        // N2 is Yin (-).
        // Sitting S2 (Wu): 180 degrees. Yin (-).

        // Base Chart Period 8:
        // Center: 8
        // NW: 9, W: 1, NE: 2, S: 3, N: 4, SW: 5, E: 6, SE: 7

        // Sitting Star (Mountain): Star at Sitting (South).
        // In Base 8 chart, South is 3.
        // Mountain Star is 3.
        // 3 corresponds to East (Mao - Yin, Jia - Yang, Yi - Yin).
        // We need the polarity of the mountain matching the house's sitting index (2).
        // Sitting is S2 (Wu, Yin). Index 2.
        // 3 is East. East-2 is Mao (Yin).
        // Star 3 (Odd/Yang). Pattern: + - -. Index 2 is -.
        // Polarity is Reverse.
        // Mountain Star 3 flies Reverse.
        // Center: 3. NW: 2, W: 1...

        // Facing Star (Water): Star at Facing (North).
        // In Base 8 chart, North is 4.
        // Water Star is 4.
        // 4 corresponds to SE. SE-2 (Xun, Yang).
        // Star 4 (Even/Yin). Pattern: - + +. Index 2 is +.
        // Polarity is Forward.
        // Water Star 4 flies Forward.
        // Center: 4. NW: 5, W: 6...

        let chart = calculate_flying_star_chart(2004, 0.0, 2024);

        assert_eq!(chart.period, 8);
        assert_eq!(chart.facing_mountain, "N (Zi)");

        // Check Center Palace (index 0)
        let center = &chart.palaces[0];
        assert_eq!(center.base_star, 8);
        assert_eq!(center.mountain_star, 3);
        assert_eq!(center.water_star, 4);

        // Check NW Palace (index 1)
        // Mtn (Reverse 3): 2
        // Wtr (Forward 4): 5
        let nw = &chart.palaces[1];
        assert_eq!(nw.mountain_star, 2);
        assert_eq!(nw.water_star, 5);
    }
}
