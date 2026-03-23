// This function returns how much ice cream there is left in the fridge.
// If it's before 22:00 (24-hour system), then 5 scoops are left. At 22:00,
// someone eats it all, so no ice cream is left (value 0). Return `Err(String::from("Invalid hour"))` if
// `hour_of_day` is higher than 23.
fn maybe_ice_cream(hour_of_day: u16) -> Result<u16, String> {
    // TODO: Complete the function body.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: Fix this test. How do you get the value contained in the
        // result?
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // Don't change this line.
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Ok(5));
        assert_eq!(maybe_ice_cream(9), Ok(5));
        assert_eq!(maybe_ice_cream(18), Ok(5));
        assert_eq!(maybe_ice_cream(22), Ok(0));
        assert_eq!(maybe_ice_cream(23), Ok(0));
        assert_eq!(maybe_ice_cream(24), Err(String::from("Invalid hour")));
        assert_eq!(maybe_ice_cream(25), Err(String::from("Invalid hour")));
    }
}