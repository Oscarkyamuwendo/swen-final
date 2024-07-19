use std::fmt;
use chrono::{DateTime, Utc, TimeZone};

// Define the TreeNode struct
#[derive(Debug, Clone)]
struct TreeNode {
    timestamp: DateTime<Utc>,
    heart_rate: u64,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(timestamp: DateTime<Utc>, heart_rate: u64) -> Self {
        TreeNode {
            timestamp,
            heart_rate,
            left: None,
            right: None,
        }
    }
}

// Define the HeartRateTree struct
#[derive(Debug)]
pub struct HeartRateTree {
    root: Option<Box<TreeNode>>,
}

impl HeartRateTree {
    pub fn new() -> Self {
        HeartRateTree { root: None }
    }

    pub fn insert(&mut self, timestamp: u64, heart_rate: u64) {
        if let Some(new_timestamp) = Utc.timestamp_opt(timestamp as i64, 0).single() {
            let new_node = Box::new(TreeNode::new(new_timestamp, heart_rate));
            if self.root.is_none() {
                self.root = Some(new_node);
            } else {
                Self::insert_node(&mut self.root, new_node);
            }
        }
    }

    fn insert_node(node: &mut Option<Box<TreeNode>>, new_node: Box<TreeNode>) {
        if let Some(ref mut current_node) = node {
            if new_node.timestamp < current_node.timestamp {
                if current_node.left.is_none() {
                    current_node.left = Some(new_node);
                } else {
                    Self::insert_node(&mut current_node.left, new_node);
                }
            } else {
                if current_node.right.is_none() {
                    current_node.right = Some(new_node);
                } else {
                    Self::insert_node(&mut current_node.right, new_node);
                }
            }
        }
    }

    pub fn average_last_minute(&self, current_time: u64) -> f64 {
        if let Some(current_dt) = Utc.timestamp_opt(current_time as i64, 0).single() {
            let one_minute_ago = current_dt - chrono::Duration::minutes(1);
            let (sum, count) = self.sum_and_count(&self.root, one_minute_ago, current_dt);
            if count == 0 {
                0.0
            } else {
                sum as f64 / count as f64
            }
        } else {
            0.0
        }
    }

    fn sum_and_count(&self, node: &Option<Box<TreeNode>>, from: DateTime<Utc>, to: DateTime<Utc>) -> (u64, u64) {
        if let Some(ref current_node) = node {
            if current_node.timestamp < from {
                return self.sum_and_count(&current_node.right, from, to);
            } else if current_node.timestamp > to {
                return self.sum_and_count(&current_node.left, from, to);
            } else {
                let (left_sum, left_count) = self.sum_and_count(&current_node.left, from, to);
                let (right_sum, right_count) = self.sum_and_count(&current_node.right, from, to);
                let total_sum = left_sum + right_sum + current_node.heart_rate;
                let total_count = left_count + right_count + 1;
                return (total_sum, total_count);
            }
        }
        (0, 0)
    }
}

// Implementing Display trait for TreeNode
impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.timestamp.to_rfc3339(), self.heart_rate)
    }
}

// Implementing Display trait for HeartRateTree
impl fmt::Display for HeartRateTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn display_node(f: &mut fmt::Formatter, node: &Option<Box<TreeNode>>) -> fmt::Result {
            if let Some(ref current_node) = node {
                writeln!(f, "{}", current_node)?;
                display_node(f, &current_node.left)?;
                display_node(f, &current_node.right)?;
            }
            Ok(())
        }
        display_node(f, &self.root)
    }
}
