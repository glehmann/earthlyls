## FROM DOCKERFILE

#### Synopsis

* `FROM DOCKERFILE [options...] <context-path>`

#### Description

The `FROM DOCKERFILE` command initializes a new build environment, inheriting from an existing Dockerfile. This allows the use of Dockerfiles in Earthly builds.

The `<context-path>` is the path where the Dockerfile build context exists. By default, it is assumed that a file named `Dockerfile` exists in that directory. The context path can be either a path on the host system, or an [artifact reference](../guides/importing.md#artifact-reference), pointing to a directory containing a `Dockerfile`.
Additionally, when using a `<context-path>` from the host system, a `.dockerignore` in the directory root will be used to exclude files (unless `.earthlyignore` or `.earthignore` are present).

#### Options

##### `-f <dockerfile-path>`

Specify an alternative Dockerfile to use. The `<dockerfile-path>` can be either a path on the host system, relative to the current Earthfile, or an [artifact reference](../guides/importing.md#artifact-reference) pointing to a Dockerfile.

{% hint style='info' %}
It is possible to split the `Dockerfile` and the build context across two separate [artifact references](../guides/importing.md#artifact-reference):

```Dockerfile
FROM alpine

mybuildcontext:
    WORKDIR /mydata
    RUN echo mydata > myfile
    SAVE ARTIFACT /mydata

mydockerfile:
    RUN echo "
FROM busybox
COPY myfile .
RUN cat myfile" > Dockerfile
    SAVE ARTIFACT Dockerfile

docker:
    FROM DOCKERFILE -f +mydockerfile/Dockerfile +mybuildcontext/mydata/*
    SAVE IMAGE testimg:latest
```

Note that `+mybuildcontext/mydata` on its own would copy the directory _and_ its contents; where as `+mybuildcontext/mydata/*` is required to copy all of the contents from within the `mydata` directory (
without copying the wrapping `mydata` directory).

If both the `Dockerfile` and build context are inside the same target, one must reference the same target twice, e.g. `FROM DOCKERFILE -f +target/dir/Dockerfile +target/dir`.
{% endhint %}

##### `--build-arg <key>=<value>`

Sets a value override of `<value>` for the Dockerfile build arg identified by `<key>`. This option is similar to the `docker build --build-arg <key>=<value>` option.

##### `--target <target-name>`

In a multi-stage Dockerfile, sets the target to be used for the build. This option is similar to the `docker build --target <target-name>` option.

##### `--platform <platform>`

Specifies the platform to build on.

For more information see the [multi-platform guide](../guides/multi-platform.md).

