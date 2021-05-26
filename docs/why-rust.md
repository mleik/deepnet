# Why Rust

For DeepNet is going to be used for an offshore test. So for that we need:
* A language that is great for a quick ish prototype.
* A language that have some safety and stability built in. Work even when tings go a bit strange.

So we chose rust with some python.

## Strengths of Rust
* The rust compiler care about safety. And works as code reviewer.
* Rust do not have runtime exceptions and enforces error handling.
* Rust have features from more dynamic languages that keeps you productive. Iterators, closure/lambdas
* The compiler guide you into a good design by forcing you to pay the cost of you choices up front.
* Rust is one of the if it compiles it most likely just works languages.

## Weakness of Rust
* The ecosystem is still young.
* Not a lot of rust developers.

## Mitigation of Weaknesses
For libraries that have not good RUST support like Azure SDK we plan to embed a Python runtime. So that we can access the excellent python ecosystem of glue code. Will still having rust deal with security and stability.

Rust is a language on the rise. So there are many rust curious developers out there. We plan to use Rust for the prototype. And have it guide your design. We will revisit the desertion later when there is a development team.

## Why not?

**Node.js** is a great language for a prototype. But it's to accepting and require a long stabilization period or a lot of tests trigger all possible runtime exceptions. We did not want to run a prototype node application offshore.

**C#.net** This may be the best language to write the final DeepNet in. But as a prototype is very heavy. And one of the team had was up to date on the latest framework that can be hard to learn and work with the first time.

**Python** We must be the only team on the planet with no significant python experience. Also se the runtime problems with node.
  