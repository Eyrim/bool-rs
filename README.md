# bool-rs

Years ago I wrote a [boolean evaluator in Java](https://github.com/Eyrim/BooleanEvaluator/tree/main), since then I have seen the light and joined
The Crab Church.

Seeing the Nom crate gave me an idea to parse expressions like that in Rust, and I wondered how long it would take me to implement in Rust,
what took literal months in Java.

The main differences between the versions:

Rust:

* Blazingly fast :)
* Uses Nom for parsing
* Syntax = `(1+0)`

Java:

* Does the parsing by hand
* Syntax = `AND(1, 0)`

The parsing difference is the reason one of these projects took months and the other took 3 hours (less if you're actually good at this language)
I didn't want to google how to do it in Java, so I struggled for ages, whereas Rust I just read the Nom docs and knocked it out no big deal.

