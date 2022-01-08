# Generic Types, Traits, and Lifetimes

Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics. Generics are abstract stand-ins for concrete types or other properties.

In Rust, it uses `traits` to define behavior in a generic way. You can combine traits with generic types to constrain a generic type to only those types that have a particular behavior, as opposed to just any type.

Lifetimes, a variety of generics that give the complier information about how references relate to each other. Lifetimes allow us to borrow values in many situations while still enabling the compiler to check that the references are valid.
