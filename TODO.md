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
* Creation of procedures with fixed number of arguments (`(define (fact n) ...)`).
* Creation of procedures with mixed number of arguments (`(define (+ a . addends) ..)`).
* Creation of procedures with any number of arguments (`(define (+ a . addends) ..)`).
* Evaluation of lambdas with fixed number of arguments (`(lambda (x y z) ...)`).
* Evaluation of lambdas with mixed of arguments (`(lambda (h . t) ...)`).
* Evaluation of lambdas with any number of arguments (`(lambda args ...)`).
* Evaluation of list procedures `list`, `length`, `pair?`, `cons`, `car`, `cdr`, `null?`, `list?`.

## Missing

* Evaluation of list procedures `caar` .. `cddddr`, `append`, `reverse`, `list-tail`, `list-ref`, `map`, `for-each`.
* Multiple expressions in `lambda` bodies.
* `lambda`s remember the environment in which they were created.
