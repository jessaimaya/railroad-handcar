/// Re-maps a number from one range to another.
pub fn map(
    n: f32,
    initial_1: f32,
    final_1: f32,
    initial_2: f32,
    final_2: f32,
    delimited: Option<bool>,
) -> f32 {
    let res = (n - initial_1) / (final_1 - initial_1) * (final_2 - initial_2) + initial_2;
    if delimited.is_none() || delimited.is_some() == false {
        return res;
    }
    if initial_2 < final_2 {
        constrain(res, initial_2, final_2)
    } else {
        constrain(res, final_2, initial_2)
    }
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
        let top = super::map(10.0, 0.0, 10.0, 0.0, 1.0, None);
        assert_eq!(top, 1.0);
        let bot = super::map(1.0, 1.0, 10.0, 0.0, 1.0, None);
        assert_eq!(bot, 0.0);
        let top_limit = super::map(200.0, 0.0, 100.0, 0.0, 1.0, Some(true));
        assert_eq!(top_limit, 1.0);
        let top_no_limit = super::map(200.0, 0.0, 100.0, 0.0, 1.0, Some(false));
        assert_eq!(top_no_limit, 1.0);
        let bot_no_limit = super::map(-1.0, 0.0, 100.0, 1.0, 0.0, Some(true));
        assert_eq!(bot_no_limit, 1.0);
    }
}
