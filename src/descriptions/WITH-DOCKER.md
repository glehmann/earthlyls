## WITH DOCKER

#### Synopsis

```Dockerfile
WITH DOCKER [--pull <image-name>] [--load [<image-name>=]<target-ref>] [--compose <compose-file>]
            [--service <compose-service>] [--allow-privileged]
  <commands>
  ...
END
```

#### Description

The clause `WITH DOCKER` initializes a Docker daemon to be used in the context of a `RUN` command. The Docker daemon can be pre-loaded with a set of images using options such as `-pull` and `--load`. Once the execution of the `RUN` command has completed, the Docker daemon is stopped and all of its data is deleted, including any volumes and network configuration. Any other files that may have been created are kept, however.

If multiple targets are referenced via `--load`, the images are built in parallel. Similarly, multiple images referenced with `--pull` will be downloaded in parallel.

The clause `WITH DOCKER` automatically implies the `RUN --privileged` flag.

The `WITH DOCKER` clause only supports the command [`RUN`](#run). Other commands (such as `COPY`) need to be run either before or after `WITH DOCKER ... END`. In addition, only one `RUN` command is permitted within `WITH DOCKER`. However, multiple shell commands may be stringed together using `;` or `&&`.

A typical example of a `WITH DOCKER` clause might be:

```Dockerfile
FROM earthly/dind:alpine-3.19-docker-25.0.5-r0
WORKDIR /test
COPY docker-compose.yml ./
WITH DOCKER \
        --compose docker-compose.yml \
        --load image-name:latest=(+some-target --SOME_BUILD_ARG=value) \
        --load another-image-name:latest=+another-target \
        --pull some-image:latest
    RUN docker run ... && \
        docker run ... && \
        ...
END
```

For more examples, see the [Docker in Earthly guide](../guides/docker-in-earthly.md) and the [Integration testing guide](../guides/integration.md).

For information on using `WITH DOCKER` with podman see the [Podman guide](../guides/podman.md)

{% hint style='info' %}
##### Note
For performance reasons, it is recommended to use a Docker image that already contains `dockerd`. If `dockerd` is not found, Earthly will attempt to install it.

Earthly provides officially supported images such as `earthly/dind:alpine-3.19-docker-25.0.5-r0` and `earthly/dind:ubuntu-23.04-docker-25.0.2-1` to be used together with `WITH DOCKER`.
{% endhint %}

{% hint style='info' %}
##### Note
Note that the cleanup phase (after the `RUN` command has finished), does not occur when using a `LOCALLY` target, users should use `RUN docker run --rm ...` to have docker remove the image after execution.
{% endhint %}

#### Options

##### `--pull <image-name>`

Pulls the Docker image `<image-name>` from a remote registry and then loads it into the temporary Docker daemon created by `WITH DOCKER`.

This option may be repeated in order to provide multiple images to be pulled.

{% hint style='info' %}
##### Note
It is recommended that you avoid issuing `RUN docker pull ...` and use `WITH DOCKER --pull ...` instead. The classical `docker pull` command does not take into account Earthly caching and so it would redownload the image much more frequently than necessary.
{% endhint %}

##### `--load [<image-name>=]<target-ref>`

Builds the image referenced by `<target-ref>` and then loads it into the temporary Docker daemon created by `WITH DOCKER`. Within `WITH DOCKER`, the image can be referenced as `<image-name>`, if specified, or otherwise by the name of the image specified in the referenced target's `SAVE IMAGE` command.

`<target-ref>` may be a simple target reference (`+some-target`), or a target reference with a build arg `(+some-target --SOME_BUILD_ARG=value)`.

This option may be repeated in order to provide multiple images to be loaded.

The `WITH DOCKER --load` option does not mark any saved images or artifacts of the referenced target for local output, nor does it mark any push commands of the referenced target for pushing. For that, please use [`BUILD`](#build).

##### `--compose <compose-file>`

Loads the compose definition defined in `<compose-file>`, adds all applicable images to the pull list and starts up all applicable compose services within.

This option may be repeated, thus having the same effect as repeating the `-f` flag in the `docker-compose` command.

##### `--service <compose-service>`

Specifies which compose service to pull and start up. If no services are specified and `--compose` is used, then all services are pulled and started up.

This option can only be used if `--compose` has been specified.

This option may be repeated in order to specify multiple services.

##### `--platform <platform>`

Specifies the platform for any referenced `--load` and `--pull` images.

For more information see the [multi-platform guide](../guides/multi-platform.md).

##### `--allow-privileged`

Same as [`FROM --allow-privileged`](#allow-privileged).

##### `--build-arg <key>=<value>` (**deprecated**)

This option is deprecated. Please use `--load <image-name>=(<target-ref> --<build-arg-key>=<build-arg-value>)` instead.

