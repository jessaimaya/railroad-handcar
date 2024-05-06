/// Re-maps a number from one range to another.
pub fn map_num(num: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    ((num - in_min) * (out_max - out_min)) / (in_max - in_min) + out_min
}

/// Constrains a number between a minimum and maximum value.
pub fn constrain(n: f32, low: f32, high: f32) -> f32 {
    let min = high.min(n);
    let max = min.max(low);
    return max;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_constrain() {
        let same = super::constrain(10.0, 0.0, 20.0);
        assert_eq!(same, 10.0);
        let hi = super::constrain(100.0, 10.0, 20.0);
        assert_eq!(hi, 20.0);
        let lo = super::constrain(0.0, 10.0, 20.0);
        assert_eq!(lo, 10.0);
    }
    #[test]
    fn test_map() {
        let top = super::map_num(10.0, 0.0, 10.0, 0.0, 1.0);
        assert_eq!(top, 1.0);
        let bot = super::map_num(1.0, 1.0, 10.0, 0.0, 1.0);
        assert_eq!(bot, 0.0);
    }
}
