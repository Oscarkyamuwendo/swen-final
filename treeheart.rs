#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_insertion() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        let current_time = 1625150300;
        assert_eq!(tree.average_last_minute(current_time), 0.0);
    }

    #[test]
    fn test_multiple_insertions() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        tree.insert(1625150250, 75);
        tree.insert(1625150300, 80);
        let current_time = 1625150300;
        assert_eq!(tree.average_last_minute(current_time), 77.5); // (75+80)/2
    }

    #[test]
    fn test_edge_case_no_data_in_interval() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 70);
        let current_time = 1625150600; // No data in the last minute
        assert_eq!(tree.average_last_minute(current_time), 0.0);
    }

    #[test]
    fn test_edge_case_single_data_point() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150540, 70);
        let current_time = 1625150600; // Only one data point in the last minute
        assert_eq!(tree.average_last_minute(current_time), 70.0);
    }

    #[test]
    fn test_display_trait() {
        let mut tree = HeartRateTree::new();
        tree.insert(1625150000, 72);
        tree.insert(1625150300, 75);
        tree.insert(1625150600, 78);

        let expected_output = "\
2021-07-01T14:33:20+00:00 - 72\n\
2021-07-01T14:38:20+00:00 - 75\n\
2021-07-01T14:43:20+00:00 - 78\n";

        let output = format!("{}", tree);
        assert_eq!(output, expected_output);
    }
}
