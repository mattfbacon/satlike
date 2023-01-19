# SAT-like

A deduction verifier. Given a set of premises, determines the validity of a deduction.

## Examples

A trivial example: we say that `a` is true, then ask if `a` is true:

```
a
∴ a
the deduction is valid.
```

An equally trivial example, but demonstrating an invalid deduction:

```
¬a
∴ a
the deduction is invalid.
```

A slightly less trivial example: we say that `a` is true and that `a` implies `b`, then ask if `b` is true:

```
a
a → b
∴ b
the deduction is valid.
```

A complex example that demonstrates all the simplification rules:

```
(m ∧ ¬b) → j
(f ∨ s) → m
b → t
f → ¬t
f
∴ j
the deduction is valid.
```
