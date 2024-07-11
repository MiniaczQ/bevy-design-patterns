# Pattern Name: Deferred Spawn

## Description

Offload entity spawning to dedicated logic.
This allows you to spawn complex entities without cluttering your spawn logic with entity asset and composition logic.

This comes in two flavours:
- Simple - Spawn and forget, good for running VFX and SFX which don't require much additional configuration.
- Complex - Spawn and keep modifying, good for spawning player, enemy, levels, which nned additional spawn context specific components like `StateScoped`.

## Implementation

[Simple](./src/simple.rs)

[Complex](./src/complex.rs)

## Use cases

- Spawn visual/sound effects.
- Control soundtrack.
- Spawn complex game entities like levels, enemies, players, interactables.

## Alternatives

- [Component Installer]()

## Credit

- Developed for the Bevy Template
