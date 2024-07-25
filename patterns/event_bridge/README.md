# Pattern Name: Event bridge

## Description

Send events between your plugins/systems without a shared event type (or other imports). This can make your plugins fully independent and exchangeable. The small amount of boilerplate per system will be dependent on the plugin, though. Making your plugins send/receive generic events allows a project to use an actual custom event type to be used in the ECS. This is done by translating before sending and after receiving an event.

## Implementation

[Implementation on both sides (event emitter and receiver plugins)](./src/main.rs)

## Use cases

This pattern is applied to plugins/systems making use of events. Such plugins can then be connected using a small amount of code within your project.

You can use it to connect plugins in a very loose way without any of them knowing anything about the other. It does not introduce additional game cycles for the event transfer, but add some conversion overhead (usually very small). The connecting event can be fully customized.

## Alternatives

- Translation systems (a system function which receives all events of a given type and for each emits another type of event)
- Accept dependencies (e.g. shared types) between (internal-only) plugins

## Credit

This makes use of [bevy's support for generics](https://bevy-cheatbook.github.io/patterns/generic-systems.html) and the [From-trait from the rust standard library](https://doc.rust-lang.org/std/convert/trait.From.html).

