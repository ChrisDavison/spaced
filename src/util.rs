use anyhow::{anyhow, Result};

pub fn next_larger_interval(i: usize, custom_intervals: &[usize]) -> usize {
    for &thing in custom_intervals {
        if thing > i {
            return thing;
        }
    }
    // We didn't find a larger interval, so use the max interval
    custom_intervals[custom_intervals.len() - 1]
}

pub fn next_smaller_interval(i: usize, custom_intervals: &[usize]) -> usize {
    let mut interval = 0;
    for &thing in custom_intervals {
        if thing > i {
            break;
        }
        interval = thing;
    }
    if interval == 0 {
        // We didn't find a smaller interval, so use the first of custom intervals
        custom_intervals[0]
    } else {
        interval
    }
}

pub fn get_custom_intervals() -> Result<Vec<usize>> {
    let mut intervals = Vec::new();
    if let Ok(interval_str) = std::env::var("SPACED_INTERVALS") {
        for thing in interval_str.split(',') {
            intervals.push(thing.trim().parse::<usize>()?);
        }
    }
    Ok(intervals)
}
