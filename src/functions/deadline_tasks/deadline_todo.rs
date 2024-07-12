use super::deadline_helpers::*;
use crate::functions::json_file_structs::*;
use std::io::Write;

// chartodo dl-a new-item 2025-01-01 00:00 > len = 3
// chartodo dl-a new-item 2025-01-01 00:00 2nd-item 2025-01-02 00:00 > len = 6

// chartodo dl-ant new-item 2025-01-01 > len = 2

// chartodo dl-and new-item 00:00 > len = 2