# SwiftDB

SwiftDB is a performant, [ACID-compliant](#acid-compliance), stripped-down
document database built to work easily with Swift.

## Syntax

SwiftDB's query language is a lisp, so the client uses s-expressions to
construct queries, like `(select sp t wn (coll people) (> (f age) 60))`. A list
of functions and their definitions are located below.

## Transaction Model

SwiftDB transactions have five stages:

-   [Open the transaction](#open-the-transaction)
-   [Select values](#select-values)
-   [Acquire locks](#acquire-locks)
-   [Perform reads and writes](perform-reads-and-writes)
-   [Commit, roll back, or close the transaction](#commit-roll-back-or-close-the-transaction)

### Open the transaction

Transactions are opened with the statement `(open t)`, where `t` may any valid
identifier. This is described in [Function Definitions](#function-definitions).

### Select values

Values are selected using the function
`(select [ident] [transaction]? [lock] [collection] [condition])`. The results
of the selection will be associated with `[ident]`. In the case that a
connection may have multiple transactions running at the same time,
`[transaction]` explicitly associates the selection with one of them. If not
present, the most recently opened transaction is used. `[lock]` specifies the
type of lock the transaction will acquires in the
[acquire locks](#acquire-locks) stage. The value `r` represents a read-only
lock. A read-only lock will wait if it contests a `wb` lock. The value `wb`
represents a blocking read/write lock. A blocking read/write lock will wait if
it contests a `wn` lock. The value `wn` represents a non-blocking read/write
lock. A non-blocking read/write lock will wait if it contests a `wb` or a `wn`
lock. `[collection]` must be the result of a call to `(coll [ident])` where
`[ident]` is the name of the targeted document collection. Finally,
`[condition]` is a prefix-notation condition expression, described in
[Query Conditions](#query-conditions) below.

### Acquire locks

Wait for locks to acquire using `(acquire [transaction])`. After all the
transaction's locks have been acquired, SwiftDB sends back a response indicating
their acquisition. While waiting, SwiftDB will parse and acknowledge any
read/write commands the client sends.

### Perform reads and writes

After locks are acquired, the client may perform reads and writes using the
functions `(create)`, `(read)`, `(update)`, `(updateall)`, `(writeall)`, and
`(delete)`. These changes are only visible to the current transaction until they
are committed.

### Commit, roll back, or close the transaction

Use `(commit [transaction])` or `(close [transaction])` to end the transaction.
Its identifier may no longer be used in any other command. If writes were
performed, `(commit)` writes them to the disk, making them visible to all
transactions opened _after_ the commit. When the client recieves the
acknowledgement, the changes were fully written to the disk and are guaranteed
to be durable (see [durability](#durability)). `(close)` ends the transaction,
discarding any changes made during the read/write phase. If no writes were
performed, this is the preferred command to end the transaction.

## Function Definitions

### Query Conditions

## ACID Compliance
