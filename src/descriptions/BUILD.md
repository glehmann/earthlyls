## BUILD

#### Synopsis

* `BUILD [options...] <target-ref> [--<build-arg-name>=<build-arg-value>...]`

#### Description

The command `BUILD` instructs Earthly to additionally invoke the build of the target referenced by `<target-ref>`, where `<target-ref>` follows the rules defined by [target referencing](../guides/importing.md#target-reference). The invocation will mark any images, or artifacts saved by the referenced target for local output (assuming local output is enabled), and any push commands issued by the referenced target for pushing (assuming pushing is enabled).

Multiple `BUILD` commands issued one after the other will be executed in parallel if the referenced targets don't depend on each other.

{% hint style='info' %}
##### What is being output and pushed

In Earthly v0.6+, what is being output and pushed is determined either by the main target being invoked on the command-line directly, or by targets directly connected to it via a chain of `BUILD` calls. Other ways to reference a target, such as `FROM`, `COPY`, `WITH DOCKER --load` etc, do not contribute to the final set of outputs or pushes.

If you are referencing a target via some other command, such as `COPY` and you would like for the outputs or pushes to be included, you can issue an equivalent `BUILD` command in addition to the `COPY`. For example

```Dockerfile
my-target:
    COPY --platform=linux/amd64 (+some-target/some-file.txt --FOO=bar) ./
```

Should be amended with the following additional `BUILD` call:

```Dockerfile
my-target:
    BUILD --platform=linux/amd64 +some-target --FOO=bar
    COPY --platform=linux/amd64 (+some-target/some-file.txt --FOO=bar) ./
```

This, however, assumes that the target `+my-target` is itself connected via a `BUILD` chain to the main target being built. If that is not the case, additional `BUILD` commands should be issued higher up the hierarchy.
{% endhint %}

#### Options

##### `--<build-arg-key>=<build-arg-value>`

Sets a value override of `<build-arg-value>` for the build arg identified by `<build-arg-key>`.

The override value of a build arg may be a constant string

```
--SOME_ARG="a constant value"
```

or an expression involving other build args

```
--SOME_ARG="a value based on other args, like $ANOTHER_ARG and $YET_ANOTHER_ARG"
```

or a dynamic expression, based on the output of a command executed in the context of the build environment.

```
--SOME_ARG=$(find /app -type f -name '*.php')
```

Dynamic expressions are delimited by `$(...)`.

##### `--platform <platform>`

Specifies the platform to build on.

This flag may be repeated in order to instruct the system to perform the build for multiple platforms. For example

```Dockerfile
build-all-platforms:
    BUILD --platform=linux/amd64 --platform=linux/arm/v7 +build
```

For more information see the [multi-platform guide](../guides/multi-platform.md).

##### `--auto-skip` (*coming soon*)

Instructs Earthly to skip the build of the target if the target's dependencies have not changed from a previous successful build. For more information on how to use this feature, see the [auto-skip section of the caching in Earthfiles guide](../caching/caching-in-earthfiles.md#auto-skip).

##### `--allow-privileged`

Same as [`FROM --allow-privileged`](#allow-privileged).

##### `--pass-args`

Same as [`FROM --pass-args`](#pass-args).

##### `--build-arg <build-arg-key>=<build-arg-value>` (**deprecated**)

This option is deprecated. Please use `--<build-arg-key>=<build-arg-value>` instead.

