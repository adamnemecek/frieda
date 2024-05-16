use std::collections::BTreeSet;

use crate::math::Set;
use crate::prelude::*;

/// A deterministic Rabin automaton (DRA) uses a [`RabinCondition`] to determine acceptance.
/// Specifically, such a condition consists of a set of [`RabinPair`]s, which in turn are
/// made up of a set `fin` and a set `inf`. A Rabin pair is now satisfied by an infinite run
/// if no color from `fin` is visited infinitely often and at least one color from `inf` is
/// visited infinitely often. Overall, a Rabin condition is then satisfied if at least one of
/// its constituent pairs is satisfied.
pub type DRA<A = CharAlphabet, C = usize> = Automaton<DTS<A, Void, C>, RabinCondition<C>, true>;
/// Helper type alias for casting a given transition system `T` into a [`DRA`].
pub type IntoDRA<T> = Automaton<T, RabinCondition<EdgeColor<T>>, true>;

/// Represents a Rabin condition, which is a set of [`RabinPair`]s. Such a condition is satisfied
/// if at least one of its pairs is satisfied.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct RabinCondition<C: Color>(Set<RabinPair<C>>);

/// A Rabin pair over some [`Color`] `C` consists of a set `fin` and a set `inf` of elements of type `C`.
/// A pair is satisfied by a set (usually the set of colors that appear infinitely often in a run),
/// if the set contains no elements of `fin` and at least one element of `inf`.
#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct RabinPair<C> {
    pub(crate) fin: BTreeSet<C>,
    pub(crate) inf: BTreeSet<C>,
}

impl<C, I> From<I> for RabinCondition<C>
where
    C: Color,
    I: IntoIterator<Item = RabinPair<C>>,
{
    fn from(value: I) -> Self {
        Self(value.into_iter().collect())
    }
}

impl<C: Color> RabinPair<C> {
    /// Creates a new pair from the given set of finite and infinite colors.
    pub fn new(fin: BTreeSet<C>, inf: BTreeSet<C>) -> Self {
        Self { fin, inf }
    }

    /// Creates a new pair from iterators giving the set of finite and infinite colors.
    pub fn from_iters<I, J>(fin: I, inf: J) -> Self
    where
        I: IntoIterator<Item = C>,
        J: IntoIterator<Item = C>,
    {
        Self {
            fin: fin.into_iter().collect(),
            inf: inf.into_iter().collect(),
        }
    }

    /// Returns true if and only if the pair is satisfied by the given set of colors, i.e.
    /// if the set contains no color from `fin` and at least one color from `inf`.
    pub fn satisfied_by_set(&self, colors: &BTreeSet<C>) -> bool {
        self.fin.intersection(colors).next().is_none()
            && self.inf.intersection(colors).next().is_some()
    }

    /// Returns true if and only if the pair is satisfied by the set of colors yielded by `iter`.
    /// This simply collects and calls [`Self::satisfied_by_set`].
    pub fn satisfied_by_iter<I: IntoIterator<Item = C>>(&self, colors: I) -> bool {
        self.satisfied_by_set(&colors.into_iter().collect())
    }
}

impl<Q, C: Color> Semantics<Q, C> for RabinCondition<C> {
    type Output = bool;
}

impl<Q, C: Color> OmegaSemantics<Q, C> for RabinCondition<C> {
    fn evaluate<R>(&self, run: R) -> Self::Output
    where
        R: OmegaRun<StateColor = Q, EdgeColor = C>,
    {
        let Some(colors) = run.recurring_edge_colors_iter() else {
            return false;
        };
        let colorset = colors.collect();

        self.0.iter().any(|pair| pair.satisfied_by_set(&colorset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rabin_pairs() {
        let pair = RabinPair::from_iters([1], [2]);
        let mut colors = vec![1, 2].into_iter().collect();
        assert!(!pair.satisfied_by_set(&colors));
        colors.remove(&1);
        assert!(pair.satisfied_by_set(&colors));
        colors.remove(&2);
        assert!(!pair.satisfied_by_set(&colors));
        colors.insert(1);
        assert!(!pair.satisfied_by_set(&colors));
    }

    #[test]
    fn rabin_automaton() {
        let ts = TSBuilder::without_state_colors()
            .with_transitions([
                (0, 'a', 0, 0),
                (0, 'b', 1, 1),
                (1, 'a', 0, 0),
                (1, 'b', 1, 1),
            ])
            .into_dts();
        let dra = DRA::from_parts_with_acceptance(ts, 0, [RabinPair::from_iters([], [1])].into());
        assert!(dra.accepts(upw!("ba")));
        assert!(!dra.accepts(upw!("a")));
        assert!(dra.accepts(upw!("ab")));
    }
}
