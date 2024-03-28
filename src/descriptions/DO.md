## DO

#### Synopsis

* `DO [--allow-privileged] <function-ref> [--<build-arg-key>=<build-arg-value>...]`

#### Description

The command `DO` expands and executes the series of commands contained within a function [referenced by `<function-ref>`](../guides/importing.md#function-reference).

Unlike performing a `BUILD +target`, functions inherit the build context and the build environment from the caller.

Functions create their own `ARG` scope, which is distinct from the caller. Any `ARG` that needs to be passed from the caller needs to be passed explicitly via `DO +MY_FUNCTION --<build-arg-key>=<build-arg-value>`.

For more information see the [Functions Guide](../guides/functions.md).

#### Options

##### `--allow-privileged`

Same as [`FROM --allow-privileged`](#allow-privileged).

##### `--pass-args`

Same as [`FROM --pass-args`](#pass-args).

