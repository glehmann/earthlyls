## SET

#### Synopsis

* `SET <name>=<value>`

#### Description

The command `SET` may be used to change the value of a previously declared variable, so long as the variable was declared with `LET`.

`ARG` variables may *not* be changed by `SET`, since `ARG` is intended to accept overrides from the CLI. If you want to change the value of an `ARG` variable, redeclare it with `LET someVar = "$someVar"` first.

See [the `LET` docs for more info](#let).

