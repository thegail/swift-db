# SwiftDB

SwiftDB is a performant,
[ACID-compliant](#acid-compliance), stripped-down
document database built to work easily with Swift.

## Syntax

SwiftDB's query language is a lisp, so the client uses
s-expressions to construct queries, like
`(select sp t wn (coll people) (> (f age) 60))`. A list of functions and their definitions are located below.

## Transaction Model

SwiftDB transactions have five stages:

-   [Open the transaction](#open-the-transaction)
-   [Select values](#select-values)
-   [Acquire locks](#acquire-locks)
-   [Perform reads and writes](perform-reads-and-writes)
-   [Commit, roll back, or close the
    transaction](#commit-roll-back-or-close-the-transaction)

### Open the transaction

Transactions are opened with the statement
`(open t)`, where `t` may any valid identifier. This is
described in [Function
Definitions](#function-definitions).

### Select values

Values are selected using the function
`(select [ident] [transaction]? [lock] [collection] [condition])`. The results of the selection will be
associated with `[ident]`. In the case that a connection
may have multiple transactions running at the same time,
`[transaction]` explicitly associates the selection with
one of them. If not present, the most recently opened
transaction is used. `[lock]` specifies the type of lock
the transaction will acquires in the
[acquire locks](#acquire-locks) stage. The value `r`
represents a read-only lock. A read-only lock will wait
if it contests a `wb` lock. The value `wb` represents a
blocking read/write lock. A blocking read/write lock will
wait if it contests a `wn` lock. The value `wn` represents a non-blocking read/write lock. A non-blocking
read/write lock will wait if it contests a `wb` or a
`wn` lock. `[collection]` must be the result of a call to
`(coll [ident])` where `[ident]` is the name of the
targeted document collection. Finally, `[condition]` is
a prefix-notation condition expression, described in
[Function Definitions](#function-definitions) below.

### Acquire locks

### Perform reads and writes

### Commit, roll back, or close the transaction

## Function Definitions

## ACID Compliance
