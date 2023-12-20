# Pattern Name: Entity Installer

## Summary

This pattern allows you to separate responsibility for attaching plugin-specific boilerplate code to entities.

## Problem

When separating responsibility into plugins you will run into an issue, where entity spawning will contain a lot of boilerplate associated with other plugins. This is problematic in 2 ways:
- Entity spawning requires knowledge (and boilerplate) of all logic that will be used, this means you may have to access a lot of plugin-specific resources too,
- Removing a plugin will break spawning code.

## Solution

Use change tracking, tags and installer systems, which specialize in adding plugin-specific boilerplate code to new entities.

To make it work, you need to define a marker component for functionality that can be installed.
This tag can be defined from the spawn system side and injected into installer system through generics.
The installer system will look for addition of this tag and apply the installation procedure.
This can consist of adding new components, new child entities as well as removing the installer tag.

[Example](./src/lib.rs)

## Cons

Installing will take one or more frames to take effect, depending on how many installers are reliant on each other.

## Alternatives

Adding all of the required components and child entities during spawning.

Helper resources/system params for providing `impl Bundle`. This would remove installation delays and separate plugin-specific boilerplate, but spawning code would still be aware of the plugins.

## Related patterns

None

## Tags

expanding entities, entity plugins, entity extensions
