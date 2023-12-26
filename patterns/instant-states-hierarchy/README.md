# Pattern Name: Instant States Hierarchy

## Summary

This pattern adds instantly propagated hierarchical states with minor API changes.

## Problem

Standard states API is good for multiple flat states, but it's dificult to use it with hierarchical states.
There doesn't really exist a good support for them.

## Solution

Add a `StateActivity<S>` middle-layer for state representation.
This can either be `Inactive` or `Active(S)`.

This layer is only visible when playing with internals:
- `State<S>` turned into `State<StateActivity<S>>`,
- `NextState<StateActivity<S>>` new.

The surface API for states does not change:
- `NextState<S>`,
- `OnExit`,
- `OnTransition`,
- `OnEnter`.

All states start as `Inactive`.
Root states get set to `Active` during the first `StateTransition` schedule.
This is a potential breaking change.
Substates are set and unset during their parent's `OnEnter` and `OnExit` schedules.

When turning `Active` both root states and substates will take a `NextState<S>` value if available and fallback to default.

All transition schedules (`OnExit`, `OnTransition`, `OnEnter`) only run if all of their states are `Active`.

System scheduling is used for instant propagation, substates are updated after their parents in the same tick.

[Example](./src/lib.rs) (check tests for use cases)

## Cons

Multiple exlusive systems one after another.
But they're extremelly tiny and run any logic rarely.

## Alternatives

- Pattern matching for nested states: https://github.com/bevyengine/bevy/pull/10088

- [Propagating states hierarchy pattern](https://github.com/MiniaczQ/bevy-design-patterns/tree/lazy-states-hierarchy)

## Tags

hierarchical states, states hierarchy, nested states
