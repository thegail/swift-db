# Query Language

SwiftDB's query language is a lisp, so the client uses s-expressions to
construct queries, like `(select sp t wn (coll people) (> (f age) 60))`. A list
of functions and their definitions are located in the language docs.

## Functions

The following top level functions are available:

-   [`(select)`](#select)
-   [`(selects)`](#select)
-   [`(acquire)`](#acquire)
-   [`(create)`](#create)
-   [`(read)`](#read)
-   [`(update)`](#update)
-   [`(readall)`](#readall)
-   [`(updateall)`](#updateall)
-   [`(delete)`](#delete)

### Select

`(select [identifier] [transaction] [lock] [collection] [condition])`.

#### `[identifier]`

The identifier the selection reference will be stored to. This cannot be a
reserved keyword or an already existing identifier. This reference can later be
used in read and write operations after acquiring locks.

#### `[transaction]`

The identifier for the transaction that this selection will take place on. This
must be a transaction which has already been opened, but which has not yet been
acquired.

#### `[lock]`

The type of lock which will be acquired on the resource when the transaction is
`(acquire)`d. The value `r` represents a read-only lock. A read-only lock will
wait if it contests a `wb` lock. The value `wb` represents a blocking read/write
lock. A blocking read/write lock will wait if it contests a `wn` lock. The value
`wn` represents a non-blocking read/write lock. A non-blocking read/write lock
will wait if it contests a `wb` or a `wn` lock.

#### `[collection]`

The collection on which the query takes place. This must be an expression
matching `(coll [name])`, where `[name]` is the name of a collection on the
database.

#### `[condition]`

A [query condition](#query-conditions) expression.

## Query Conditions

A query condition.
