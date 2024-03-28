## FROM

#### Synopsis

* `FROM <image-name>`
* `FROM [--platform <platform>] [--allow-privileged] <target-ref> [--<build-arg-key>=<build-arg-value>...]`

#### Description

The `FROM` command initializes a new build environment and sets the base image for subsequent instructions. It works similarly to the classical [Dockerfile `FROM` instruction](https://docs.docker.com/engine/reference/builder/#from), but it has the added ability to use another [target](https://docs.earthly.dev/docs/guides/target-ref#target-reference)'s image as the base image.

Examples:

* Classical reference: `FROM alpine:latest`
* Local reference: `FROM +another-target`
* Relative reference: `FROM ./subdirectory+some-target` or `FROM ../otherdirectory+some-target`
* Absolute reference: `FROM /absolute/path+some-target`
* Remote reference from a public or [private](https://docs.earthly.dev/docs/guides/auth) git repository: `FROM github.com/example/project+remote-target`

The `FROM` command does not mark any saved images or artifacts of the referenced target for output, nor does it mark any push commands of the referenced target for pushing. For that, please use [`BUILD`](#build).

{% hint style='info' %}
##### Note

The `FROM ... AS ...` form available in the classical Dockerfile syntax is not supported in Earthfiles. Instead, define a new Earthly target. For example, the following Dockerfile

```Dockerfile
# Dockerfile

FROM alpine:3.18 AS build
# ... instructions for build

FROM build as another
# ... further instructions inheriting build

FROM busybox as yet-another
COPY --from=build ./a-file ./
```

can become

```Dockerfile
# Earthfile

build:
    FROM alpine:3.18
    # ... instructions for build
    SAVE ARTIFACT ./a-file

another:
    FROM +build
    # ... further instructions inheriting build

yet-another:
    FROM busybox
    COPY +build/a-file ./
```
{% endhint %}

#### Options

##### `--<build-arg-key>=<build-arg-value>`

Sets a value override of `<build-arg-value>` for the build arg identified by `<build-arg-key>`. See also [BUILD](#build) for more details about build args.

##### `--platform <platform>`

Specifies the platform to build on.

For more information see the [multi-platform guide](../guides/multi-platform.md).

##### `--allow-privileged`

Allows remotely-referenced targets to request privileged capabilities; this flag has no effect when referencing local targets.

Additionally, for privileged capabilities, earthly must be invoked on the command line with the `--allow-privileged` (or `-P`) flag.

For example, consider two Earthfiles, one hosted on a remote GitHub repo:

```Dockerfile
# github.com/earthly/example
FROM alpine:latest
elevated-target:
    RUN --privileged echo do something requiring privileged access.
```

and a local Earthfile:

```Dockerfile
FROM alpine:latest
my-target:
    FROM --allow-privileged github.com/earthly/example+elevated-target
    # ... further instructions inheriting remotely referenced Earthfile
```

then one can build `my-target` by invoking earthly with the `--allow-privileged` (or `-P`) flag:

```bash
earthly --allow-privileged +my-target
```

##### `--pass-args`

Earthly automatically passes all current arguments to referenced targets in the _same_ Earthfile.
However, when the `--pass-args` flag is set, Earthly will also propagate all arguments to an externally referenced target.

##### `--build-arg <key>=<value>` (**deprecated**)

This option is deprecated. Use `--<build-arg-key>=<build-arg-value>` instead.

