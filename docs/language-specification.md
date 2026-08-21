# VietLang 0.3.0-alpha.1 language specification (draft)

Status: descriptive draft for the Rust reference interpreter. This document
does not claim a stable language specification. Where interpreter and VM differ,
the interpreter defines current behavior and the VM must reject unsupported AST
nodes explicitly.

## Source and lexical rules

- Source files are UTF-8 and conventionally use `.vl`.
- Identifiers may contain Unicode in the Rust lexer. Keywords are ASCII and
  case-sensitive.
- `//` starts a line comment; `/* ... */` is a block comment.
- Newlines and semicolons separate statements. The parser discards newline
  tokens, so newline placement does not change expression precedence.
- Integer literals are signed 64-bit values after parsing. Floating-point
  literals use IEEE-754 binary64.

## Bindings and scopes

- `let name = value` creates an immutable binding.
- `let mut name = value` creates a mutable binding.
- Reading an unknown name or assigning an unknown/immutable name is an error.
- Blocks and function calls create lexical scopes. A declaration shadows an
  outer binding in its inner scope.
- Initializers and function arguments are evaluated left-to-right.

## Runtime values

The interpreter uses dynamic runtime values: `Int`, `Float`, `String`, `Bool`,
`None`, `Array`, struct/map values, enum variants, ranges, functions, and builtin
functions. Before execution, the gradual semantic analyzer enforces annotations
when both sides are locally known. Imported and native values without a declared
signature have type `Unknown`, which is compatible with every type. Consequently,
a successful check is not yet a sound whole-program static-safety guarantee.

`None` is false in conditions. `false`, numeric zero, empty strings, and empty
arrays are also false; other values are true.

## Functions

- Arguments are evaluated before the call.
- Calls must supply every parameter without a default and may not exceed the
  declared parameter count.
- Default expressions are evaluated in the function call environment when the
  corresponding argument is omitted.
- Fixed-arity builtins reject both missing and extra arguments. Builtins marked
  variadic validate their own accepted ranges.
- `return value` exits the nearest function. Falling off the end returns
  `None`.
- Locally known argument, default-value, and return annotations are checked
  before execution. A function with a declared non-`None` return type must not
  fall through its body.
- Lambdas accept an optional return annotation, for example
  `fn(value: Int) -> Int { return value * 2 }`. Without it, locally observed
  `return` values determine the inferred function return type.

## Lexical closures

A function or lambda captures the lexical bindings visible at its declaration.
The captured bindings remain alive after their declaring block returns. Sibling
closures capture the same binding cell, so a mutation performed through one is
visible through the other. Caller-local variables never replace a captured name.
Named functions bind themselves while executing, enabling direct recursion;
module declarations added later remain visible for forward calls.

Captured binding cells are synchronized so closures may cross an OS-thread
boundary without a Rust data race. A language-level read followed by a later
write is not one atomic operation, however. Programs requiring atomic compound
state transitions must use the mutex/channel APIs.

## Operators and evaluation

- Arithmetic supports numeric operands; `+` also concatenates strings.
- Mixed `Int`/`Float` comparisons promote the integer to `Float`.
- `&&` and `||` short-circuit from left to right.
- Division by integer zero is an error. Floating-point behavior follows Rust
  `f64` behavior until a stricter numeric specification is adopted.

## Structs, enums, Option, Result, and match

Local struct literals must provide exactly the declared fields and field values
must be assignable to their annotations. `impl Type` methods must declare
`self` first; calls omit that implicit receiver and are checked against the
remaining parameters. A known struct field may only be mutated through a
mutable base binding.

Tuple enum variants are constructor functions whose payload count and locally
known payload types are checked. `match` tries arms in source order and `_` is
the wildcard pattern. Matches over `Bool` and locally declared enums must cover
every case unless a wildcard/variable arm exists; enum pattern payloads receive
their declared types. `Option<T>` and `Result<T,E>` are built-in algebraic data
types. `Some(value)`, `None`, `Ok(value)`, and `Err(error)` infer their payload
positions; pattern bindings substitute concrete payload types, and matches must
cover both variants unless they contain a wildcard. Arbitrary user-defined
generic type parameters are not yet supported.

## Errors

Lexer, parser, name, type, and runtime errors carry line and column information.
`try { ... } catch error { ... }` catches language errors. VM compilation must
return an error with a source position for every unsupported statement or
expression; silently omitting an AST node is invalid.

## Concurrency and memory model

`spawn` creates an operating-system thread with a cloned interpreter and invokes
the supplied closure in its captured lexical environment. It is not a green
thread or async task. Channels and mutex-backed native services are the explicit
synchronization mechanisms. No data-race-free static type guarantee is currently
provided.

## Modules

`import` resolves project modules, the bundled `std/` directory, or the
installed `VIETLANG_STD` directory. Before execution, the frontend constructs a
canonical dependency graph, rejects missing imports and cycles, checks every
module, and lowers declarations to typed IR. Package installation requires an
exact resolved version, immutable Git revision, SHA-256 verification,
Ed25519-signed metadata, and a versioned `vietlang.lock`.

## Compatibility requirements

A construct advertised as supported by both execution engines must produce the
same observable value/output or the same class of error in interpreter and VM.
Automated differential tests currently cover literals, bindings, assignment,
arithmetic, numeric/string comparison, unary operators, lexical shadowing,
`if`, `while`, and short-circuit Boolean operators. Extending that suite to every
future shared construct remains a release requirement.
