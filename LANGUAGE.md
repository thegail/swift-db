# Query Language

SwiftDB's query language is a lisp, so the client uses s-expressions to
construct queries, like `(select sp t wn (coll people) (> (f age) 60))`. A list
of functions and their definitions are located in the language docs.

## Functions

The following top level functions are available:

-   [`(open)`](#open)
-   [`(acquire)`](#acquire)
-   [`(commit)`](#commit)
-   [`(close)`](#close)
-   [`(select)`](#select)
-   [`(selects)`](#select)
-   [`(create)`](#create)
-   [`(read)`](#read)
-   [`(update)`](#update)
-   [`(readall)`](#read-all)
-   [`(updateall)`](#updateall)
-   [`(delete)`](#delete)

### Open

`(open [identifier])`

This opens a transaction, binding it to `identifier`. The transaction is
initialized in the selection stage.

### Acquire

`(acquire [transaction])`

Acquires locks on the selections in `transaction`, waiting if necessary.
`(acquire)` may only be called during the selection stage of a transaction.
After the acquisition is acknowledged, the transaction moves into the read/write
stage, in which the client may perform reads and writes on the transaction.

### Commit

`(commit [transaction])`

Commits any writes performed during the read/write stage of `transaction`. This
may only be called during the read/write stage of the transaction. Once the
commitment is acknowledged, it is durable and non-volatile.

### Close

`(close [transaction])`

Closes `transaction`, discarding any writes performed during the read/write
stage. If the transaction did not perform any writes, this is the preferred
method of ending it.

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

### Read All

`(readall [selection])`

Read all fields of `selection`. If `selection` is a single selection, this
returns an object with all fields of `selection`. If it is a multiple selection,
this returns an array of objects with all fields of `selection`.

## Query Conditions

A query condition.
