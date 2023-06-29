use std::ops::Range;

/// Returns a range containing the minimum and maximum values in the iterator
pub fn iter_range<T: Ord + Clone>(iter: impl IntoIterator<Item = T>) -> Option<Range<T>> {
    let mut iter = iter.into_iter();
    let Some(mut range) = iter.next().map(|e| Range {
        start: e.clone(),
        end: e
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
pub fn iter_combine_ranges<T: Ord>(iter: impl IntoIterator<Item = Range<T>>) -> Option<Range<T>> {
    let mut iter = iter.into_iter();
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

pub trait FloatRangeExt<T> {
    #[cfg(feature = "float-ord")]
    fn float_ord(&self) -> Range<float_ord::FloatOrd<T>>;
}

macro_rules! impl_float_range_ext {
    ($typ:ty) => {
        impl FloatRangeExt<$typ> for Range<$typ> {
            #[cfg(feature = "float-ord")]
            fn float_ord(&self) -> Range<float_ord::FloatOrd<$typ>> {
                Range {
                    start: float_ord::FloatOrd(self.start),
                    end: float_ord::FloatOrd(self.end)
                }
            }
        }
    }
}
impl_float_range_ext!(f32);
impl_float_range_ext!(f64);

#[cfg(feature = "float-ord")]
pub mod float_ord {
    pub(super) use float_ord::FloatOrd;
    use std::ops::Range;
    use super::FloatRangeExt;

    pub trait RangeFloatOrdExt<T: Copy> {
        fn float_range(self) -> Range<T>;
    }
    impl <T: Copy> RangeFloatOrdExt<T> for Range<FloatOrd<T>> {
        fn float_range(self) -> Range<T> {
            Range {
                start: self.start.0,
                end: self.end.0
            }
        }
    }

    macro_rules! impl_iter_range_float_ord {
        ($iter_range_name:ident, $iter_combine_ranges_name:ident, $typ:ty) => {
            /// See: [super::iter_range]
            pub fn $iter_range_name<T: Into<$typ>>(iter: impl IntoIterator<Item = T>) -> Option<Range<$typ>> {
                super::iter_range(iter.into_iter().map(|f| f.into()).map(FloatOrd))
                    .map(RangeFloatOrdExt::float_range)
            }

            /// See: [super::iter_combine_ranges]
            pub fn $iter_combine_ranges_name(iter: impl IntoIterator<Item = Range<$typ>>) -> Option<Range<$typ>> {
                super::iter_combine_ranges(iter.into_iter().map(|f| f.float_ord()))
                    .map(RangeFloatOrdExt::float_range)
            }
        }
    }

    impl_iter_range_float_ord!(iter_range_float_ord_f32, iter_combine_ranges_float_ord_f32, f32);
    impl_iter_range_float_ord!(iter_range_float_ord_f64, iter_combine_ranges_float_ord_f64, f64);

    #[cfg(test)]
    mod tests {
        use crate::core::range::float_ord::iter_range_float_ord_f32;

        #[test]
        fn test_iter_range_float_ord_f32() {
            assert_eq!(iter_range_float_ord_f32(Vec::<f32>::new()), None);
            assert_eq!(iter_range_float_ord_f32(vec![
                0.0,
                1.0,
                2.0
            ]), Some(0.0..2.0));
            assert_eq!(iter_range_float_ord_f32(vec![
                -5.4,
                2.3
            ]), Some(-5.4..2.3));
            assert_eq!(iter_range_float_ord_f32(vec![1.0]), Some(1.0..1.0));
        }
    }
}

