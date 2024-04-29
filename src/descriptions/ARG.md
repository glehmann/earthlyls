## ARG

#### Synopsis

* `ARG [--required] <name>[=<default-value>]` (constant form)
* `ARG [--required] <name>=$(<default-value-expr>)` (dynamic form)

#### Description

The command `ARG` declares a build argument (or arg) with the name `<name>` and with an optional default value `<default-value>`. If no default value is provided, then empty string is used as the default value.

This command works similarly to the [Dockerfile `ARG` command](https://docs.docker.com/engine/reference/builder/#arg), with a few differences regarding the scope and the predefined args (called builtin args in Earthly). The arg's scope is always limited to the recipe of the current target or command and only from the point it is declared onward. For more information regarding builtin args, see the [builtin args page](./builtin-args.md).

In its *constant form*, the arg takes a default value defined as a constant string. If the `<default-value>` is not provided, then the default value is an empty string. In its *dynamic form*, the arg takes a default value defined as an expression. The expression is evaluated at run time and its result is used as the default value. The expression is interpreted via the default shell (`/bin/sh -c`) within the build environment.

The value of an arg can be overridden either from the `earthly` command

```bash
earthly <target-ref> --<name>=<override-value>
```

or from a command from another target, when implicitly or explicitly invoking the target containing the `ARG`

```Dockerfile
BUILD <target-ref> --<name>=<override-value>
COPY (<target-ref>/<artifact-path> --<name>=<override-value>) <dest-path>
FROM <target-ref> --<name>=<override-value>
```

for example

```Dockerfile
BUILD +binary --NAME=john
COPY (+binary/bin --NAME=john) ./
FROM +docker-image --NAME=john
```

For more information on how to use build args see the [build arguments and variables guide](../guides/build-args.md). A number of builtin args are available and are pre-filled by Earthly. For more information see [builtin args](./builtin-args.md).

#### Options

##### `--required`

A required `ARG` must be provided at build time and can never have a default value. Required args can help eliminate cases where the user has unexpectedly set an `ARG` to `""`.

```
target-required:
    # user must supply build arg for target
    ARG --required NAME

build-linux:
    # or explicitly supply in build command
    BUILD +target-required --NAME=john
```

#### `--global`

A global `ARG` is an arg that is made available to all targets in the Earthfile. This is useful for setting a default value for an arg that is used in many targets.

Global args may only be declared in base targets.

{% hint style='danger' %}
##### Important
Avoid using `ARG --global` for args that change frequently (e.g. git sha, branch name, PR number, etc). Any change to the value of this arg would typically cause all targets in the Earthfile to re-execute with no cache.

It's always best to declare args as deep and late as possible within the specific target where they are needed, to get the most performance, even if this may require more verbose passing of args from one target to another. See also [`BUILD --pass-args`](#build).
{% endhint %}

