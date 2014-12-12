# Numbers

## Implemented

* Signed (64 bits) integers.

## Missing

* Unlimited sized signed integers.
* Limited sized rationals.
* Unlimited sized rationals.
* IEEE-754 floating point numbers.
* Complex numbers.
* Exactness.

# Parser

## Implemented

* Identifiers (atoms).
* Signed integers.
* Lists.
* Lists with `[]`.
* Booleans.
* Quoting.
* Dotted-lists.

## Missing

* Characters.
* Strings.
* Vectors.
* Bytevectors.
* Unquoting.

# Evaluation

## Implemented

* Evaluation of primitives (integers and booleans).
* Evaluation of quoted values.
* Application of `+`, `-`, `*`, `/`.
* Application of `=`, `<`, `>`, `<=`, `>=` and `not`.
* Evaluation of special forms `and` and `or`.
* Evaluation of special form `if`.
* Creation of variables (`(define pi ...)`).
* Creation of procedures with arguments (`(define (fact n) ...)`).

## Missing

* Creation of procedures with variable arguments (`(define (+ . addends) ..)`).
