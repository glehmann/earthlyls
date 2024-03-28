## IMPORT

#### Synopsis

* `IMPORT [--allow-privileged] <earthfile-ref> [AS <alias>]`

#### Description

The command `IMPORT` aliases an Earthfile reference (`<earthfile-ref>`) that can be used in subsequent [target, artifact or command references](../guides/importing.md).

If not provided, the `<alias>` is inferred automatically as the last element of the path provided in `<earthfile-ref>`. For example, if `<earthfile-ref>` is `github.com/foo/bar/buz:v1.2.3`, then the alias is inferred as `buz`.

The `<earthfile-ref>` can be a reference to any directory other than `.`. If the reference ends in `..`, then mentioning `AS <alias>` is mandatory.

If an `IMPORT` is defined in the `base` target of the Earthfile, then it becomes a global `IMPORT` and it is made available to every other target or command in that file, regardless of their base images used.

For more information see the [importing guide](../guides/importing.md).

#### Options

##### `--allow-privileged`

Similar to [`FROM --allow-privileged`](#allow-privileged), extend the ability to request privileged capabilities to all invocations of the imported alias.

