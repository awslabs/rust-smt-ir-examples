## rust-smt-ir

This project provides examples of using a Rust intermediate representation (IR) in Rust for
[SMT-LIB](http://smtlib.cs.uiowa.edu/about.shtml).  To demonstrate the benefit to the automated
reasoning community, the project includes three sample applications:

 1. A tool to perform homomorphic transformations on SMT-LIB queries,
 with a focus on string theory. String function applications are
 extracted into variables, and variable names are canonicalized. Most
 importantly, the string literals themselves are modified. The string
 properties that have to be maintained when transforming a string
 literal depend on the context in which this literal is used; this
 context is determined through a callgraph representation of the SMT
 query. The string properties themselves and the string functions they
 depend on are described in the
 [string_fcts](amzn-smt-string-transformer/src/string_fcts.rs)
 module.  The tool and its usage is described
 [here](amzn-smt-string-transformer).

 2. A tool to transform SMT-LIB queries in supported subsets of the
 language into
 [SAT](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem)
 problems.  The tool and its usage is described
 [here](amzn-smt-eager-arithmetic).
 The IR is described/documented
 [here](amzn-smt-ir).

 3. A tool to translate between an older "dialect" of SMT into the
 current standard. The tool and its usage are described
 [here](amzn-smt-string-fct-updater).

## Security

See [CONTRIBUTING](CONTRIBUTING.md#security-issue-notifications) for more information.

## License

This project is licensed under the Apache-2.0 License.

