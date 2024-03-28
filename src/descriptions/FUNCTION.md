## FUNCTION

#### Synopsis

* `FUNCTION`

#### Description

{% hint style='hint' %}
#### UDCs have been renamed to Functions

Functions used to be called UDCs (User Defined Commands). Earthly 0.7 uses `COMMAND` instead of `FUNCTION`.
{% endhint %}

The command `FUNCTION` marks the beginning of a function definition. Functions are reusable sets of instructions that can be inserted in targets or other functions. In order to reference and execute a function, you may use the command [`DO`](#do).

Unlike performing a `BUILD +target`, functions inherit the build context and the build environment from the caller.

Functions create their own `ARG` scope, which is distinct from the caller. Any `ARG` that needs to be passed from the caller needs to be passed explicitly via `DO +MY_FUNCTION --<build-arg-key>=<build-arg-value>`.

Global imports and global args are inherited from the `base` target of the same Earthfile where the command is defined in (this may be distinct from the `base` target of the caller).

For more information see the [Functions Guide](../guides/functions.md).

