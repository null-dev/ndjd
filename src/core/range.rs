use std::ops::Range;

/// Returns a range containing the minimum and maximum values in the iterator
fn iter_range<T: Ord + Clone>(mut iter: impl Iterator<Item = T>) -> Option<Range<T>> {
    let Some(mut range) = iter.next().map(|e| Range {
        start: e,
        end: e.clone()
    }) else {
        return None;
    };
    for entry in iter {
        if entry < range.start {
            range.start = entry;
        } else if entry > range.end {
            range.end = entry;
        }
    }
    Some(range)
}

/// Returns the minimum range required to contain all the input ranges
fn iter_combine_ranges<T: Ord>(mut iter: impl Iterator<Item = Range<T>>) -> Option<Range<T>> {
    let Some(mut merged_range) = iter.next() else {
        return None;
    };
    for entry in iter {
        if entry.start < merged_range.start {
            merged_range.start = entry.start;
        }
        if entry.end > merged_range.end {
            merged_range.end = entry.end;
        }
    }
    Some(merged_range)
}