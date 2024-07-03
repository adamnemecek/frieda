use crate::{prelude::*, transition_system::run::Observer};
/// This is the base trait for different types of semantics that are used by the
/// [`Automaton`] struct for determining the output of a finite or infinite
/// run. This can either be some arbitrary output in the case of a Moore or
/// Mealy machine. Or it could be boolean indicating whether the input is
/// accepted or not in case of an acceptor like a DFA.
///
/// Generally, we distinguish between an [`Automaton`] of finite and one of
/// infinite words. The purpose of a semantic is to determine what to do
/// with the run that is induced by an input on some transition system.
/// See [`Semantics`] for more details on the
/// semantics of finite and infinite word acceptors, respectively.
///
/// # Finite inputs
/// For a finite input such as a [`FiniteWord`] we use [`Semantics`],
/// which defines an `Output` type and provides the [`Semantics::evaluate`] method.
/// It takes the [`Run`] that is the result of running the finite
/// input in some transition system and turns it into the desired output.
///
/// Examples of this semantic are for example the [`MooreSemantics`], which
/// for a finite word `w` simply produce the color of the state that is
/// reached when running `w` in the transition system from the initial state.
/// This is similar to the [`ReachabilityCondition`], which additionaly demand that
/// the state colors are `bool`.
/// Further, there is also the [`MealySemantics`], which outputs the last
/// transition that is taken when reading `w`.
///
/// # Infinite inputs
/// For an infinite input like an [`OmegaWord`], we also need to define an
/// `Output` type. This is now computed in the [`Semantics::evaluate`] method on
/// the [`Semantics`] trait. It does this based on a [`Run`].
///
/// Examples include the [`BuchiCondition`], which may be applied to `bool`
/// edge-colored transition systems. It outputs `true` if any edge labeled
/// with `true` is visited infinitely often and `false` otherwise.
/// This can actually be viewed as an instantiation of the [`MinEvenParityCondition`]
/// semantics that a [`DPA`] uses, which outputs the least priority/color
/// among those that are labels of edges taken infinitely often.
pub trait Semantics<T: TransitionSystem, const OMEGA: bool = false> {
    /// The type of output that this semantic produces.
    type Output;
    /// The observer whose output is used for computing the output.
    type Observer: Observer<T>;

    /// Compute the output for the given finite run.
    fn evaluate(&self, observed: <Self::Observer as Observer<T>>::Current) -> Self::Output;
}
