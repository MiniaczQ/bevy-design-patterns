> [!Important]
> Due to rapid development of Bevy, which often invalidates existing patterns
> as well as difficulty of coming up with new ones,
> this repository will be deactivated until appropriate.
> (Feel free to contact me and take over, if you think it's the right time)
>
> If you're looking for up-to-date information, please visit the following:
> - [Bevy Best Practices](https://github.com/tbillington/bevy_best_practices)
> - [Bevy Template 2D](https://github.com/TheBevyFlock/bevy_new_2d)


> [!NOTE]
> This is a proposal for organized storage of Bevy patterns.
> The idea may not be accepted by the community and the formats can change.
> During this phase, no patterns will be merged and will only exist in PRs, so please check them for actual content!

# Bevy Design Patterns

This repository contains a handful of design patterns for Bevy.

While some content may be extremelly similar to Bevy's examples, the goal of this repository is not to showcase Bevy API, but show how it can be used to solve specific problems.

Feel free to use those in your project or contribute your own!

## What is a Design Pattern?

From Wikipedia:

```
A software design pattern is a general, reusable solution to a commonly occurring problem within a given context in software design. It is not a finished design that can be transformed directly into source or machine code. Rather, it is a description or template for how to solve a problem that can be used in many different situations. Design patterns are formalized best practices that the programmer can use to solve common problems when designing an application or system.
```

Patterns allow us to quickly solve known problems in a way that's already thought through and battle tested.

Design patterns in the context of Bevy are tighly coupled to the API and will change together with it.

To show example usage and validate patterns against Bevy API, we also ask for a valid implementation and test suite that verifies the solution's goal.

**Bevy Design Patterns are generic solutions to common problems, they need to address actual use cases, and be small enough to be testable**

* **Not every solution is a design pattern**: Majority of the solutions apply to very specific cases. While an example is important, design patterns have to generalize.

* **BDPs are not help requests**: Avoid creating BDPs for discussing solution to your problem. If You need help consider asking in the official [Bevy Discord](https://discord.com/invite/bevy) or open an Issue with a problem solving request.

* **BDPs should be small**: Small piece of code is easy to test and can be more generic. Try to split your proposal into small, standalone solutions. Feel free to create multiple BDPs if you have multiple proposals.

* **BDPs require implementations**: They contain both, an example piece of application and a test suite that verifies the problem has been solved.

* **Don't create DBP before you have an example**: If you want to explore design spaces with the Bevy community, consider finding or creating an Issue. If at any point during the discussion you discover a design you believe in enough, create a BDP. An initial Draft BDP should at a bare minimum contain example code and describe the problem it solves. BDPs are a platform for community members to share generic solutions to common problems, not for people to open discussions with the intent to eventually find a design.

If you are uncertain if you should create an BDP for your solution, don't hesitate to ask in the Issues.

## Why create an BDP?

**BDPs are intended to be a learning resource, not a burden for users.**

BDPs centralize and standardize knowledge about solutions to common problems.

BDPs server as a form of documentation. They describe how a generic problem can be solved, what are the benefits and downsides of said solution. The accompanying tests allow for quickly verifying validity of patterns against newer Bevy versions.

They don't need to be perfect, complete, or even very good when you submit them. The goal is to move the discussion into a format where we can give each part of the design the focus it deserves in a collaborative fashion.

## The Process

1. Fork this repository and create a new branch for your new BDP.
1. Copy `template` into the `patterns` folder and rename it to `my_pattern`, where `my_pattern` is a unique identifier for your pattern.
1. Fill out the BDP template with your implementation and description of `my_pattern`.
1. [Create a pull request](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/creating-a-pull-request) in this repo. The first comment should include:
   1. A one-sentence description of what the BDP is about.
   1. A link to the "rendered" form of the `README.md` file. To do so, link directly to the file on your own branch, so then the link stays up to date as the file's contents changes. See #1 for an example of what this looks like.
1. Add your proposal to the `CONTENT.md` document.
1. Help us discuss and refine the BDP. Bevy users will leave comments and suggestions. Ideally at some point relative consensus will be reached. Your BDP is accepted if your pull request is merged. If your BDP is accepted, move on to step 7. A closed BDP indicates that the design cannot be accepted in its current form.
1. Bevy users can now easily find and learn from your design pattern.

## Collaborating

First, make sure you always abide by the [Bevy Code of Conduct](https://github.com/bevyengine/bevy/blob/main/CODE_OF_CONDUCT.md) when participating in the BDP process.

Additionally, here are some suggestions to help make collaborating on BDPs easier:

* The [insert a suggestion](https://docs.github.com/en/github/collaborating-with-issues-and-pull-requests/commenting-on-a-pull-request#adding-line-comments-to-a-pull-request) feature of GitHub is extremely convenient for making and accepting quick changes.
* If you want to make significant changes to someone else's BDP, consider creating a pull request in their fork/branch. This gives them a chance to review the changes. If they merge them, your commits will show up in the original BDP PR (and they will retain your authorship).
* Try to have design discussions inside the BDP PR. If any significant discussion happens in other repositories or communities, leave a comment with a link to it in the BDP PR (ideally with a summary of the discussion).
