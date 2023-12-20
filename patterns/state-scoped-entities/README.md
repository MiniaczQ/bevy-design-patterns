# Pattern Name: State-scoped Entities

## Summary

This pattern allows for binding entity lifetime to a specific state.

## Problem

It's difficult to manage individual entity lifetimes.

## Solution

By binding entities during spawning to specific states we ensure they are cleaned up properly.

[Example](./src/lib.rs)

## Cons

If entities are created during the wrong stage, they only get cleaned up during state change.

## Alternatives

Managing all entity lifetimes manually or through cleanup systems for each.

## Related patterns

Nonw

## Tags

entity cleanup, scoping
