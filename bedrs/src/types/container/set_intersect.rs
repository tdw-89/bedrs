use crate::{
    traits::{ChromBounds, IntervalBounds, ValueBounds},
    types::Query,
    Intersect, IntervalContainer,
};

impl<'a, I, C, T> IntervalContainer<I, C, T>
where
    I: IntervalBounds<C, T>,
    C: ChromBounds,
    T: ValueBounds,
{
    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `other`
    ///
    /// # Panics
    /// Panics if the intersection of two intervals fails to be computed (like from unsorted
    /// containers)
    pub fn ix_set_target<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C, T>,
        method: Query<T>,
    ) -> Box<dyn Iterator<Item = Iv> + 'a>
    where
        Iv: IntervalBounds<C, T> + 'a,
        &'a Iv: Intersect<C, T>,
    {
        let ix_iter = self.records().iter().flat_map(move |iv| {
            let overlaps = other
                .query_iter(iv, method)
                .expect("Failed to find overlaps with provided query method")
                .cloned();
            overlaps.into_iter().map(|ov| match iv.intersect(&ov) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            })
        });
        Box::new(ix_iter)
    }

    /// Find the intersection of two sets of intervals.
    ///
    /// Returns the intersection of each interval in `self` with each interval in `other`
    /// as an iterator of interval type from `self`
    ///
    /// # Panics
    /// Panics if the intersection of two intervals fails to be computed (like from unsorted
    /// containers)
    pub fn ix_set_query<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C, T>,
        method: Query<T>,
    ) -> Box<dyn Iterator<Item = I> + 'a>
    where
        Iv: IntervalBounds<C, T> + 'a,
    {
        let ix_iter = self.records().iter().flat_map(move |iv| {
            let overlaps = other
                .query_iter(iv, method)
                .expect("Failed to find overlaps with provided query method");
            overlaps.into_iter().map(|ov| match ov.intersect(iv) {
                Some(x) => x,
                None => panic!("Interval intersection failed"),
            })
        });
        Box::new(ix_iter)
    }

    /// Find the intersection of two sets of intervals with customized metadata handling.
    ///
    /// Returns intersections with interval type determined by the combiner function
    pub fn ix_set_query_with<Iv>(
        &'a self,
        other: &'a IntervalContainer<Iv, C, T>,
        method: Query<T>,
        combiner: fn(&I, &I) -> I,
    ) -> Box<dyn Iterator<Item = I> + 'a>
    where
        Iv: IntervalBounds<C, T> + 'a,
    {
        let ix_iter = self.records().iter().flat_map(move |iv| {
            let overlaps = other
                .query_iter(iv, method)
                .expect("Failed to find overlaps with provided query method");
            overlaps.into_iter().filter_map(move |ov| {
                ov.intersect(iv).map(|x| combiner(iv, &x))
            })
        });
        Box::new(ix_iter)
    }
}