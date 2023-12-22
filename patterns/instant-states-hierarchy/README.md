# Pattern Name: Instant States Hierarchy

## Summary

This pattern adds instantly propagated hierarchical states with minor API changes.

## Problem

Standard states API is good for multiple flat states, but it's difficult to use it with hierarchical states.
There doesn't really exist a good support for them.

## Solution

Add an `Option<T>`-like enum for substates which may or may not be 'valid' depending on the parent state.
It's called `SubState` with variants `Active`, `Inactive`.
Every state is wrapped in it by default.

Root states are `Inactive` on startup, but get set during the first `StateTransition` schedule.
Substates are set (or unset) during their parent's `OnEnter` and `OnExit` schedules.
All transition schedules (`OnExit`, `OnTransition`, `OnEnter`) __do not run__ if any of their state is `Inactive`.

To ensure instant propagation system scheduling is used.

I separated the transition running systems during development, but we should get away with a single system for transition.
That's because substates need to run their schedules AFTER parent's `OnEnter` (last transition), so their states update same tick.

[Example](./src/lib.rs)

## Cons

Multiple exlusive systems one after another. But they're extremelly tiny and run any logic rarely.

API is filled with `SubState`, this can be polished during integration to Bevy.

## Alternatives

- Pattern matching for nested states: https://github.com/bevyengine/bevy/pull/10088

- [Propagating states hierarchy pattern](https://github.com/MiniaczQ/bevy-design-patterns/tree/lazy-states-hierarchy)

## Tags

hierarchical states, states hierarchy, nested states
