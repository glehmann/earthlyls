## LOCALLY

#### Synopsis

* `LOCALLY`

#### Description

The `LOCALLY` command can be used in place of a `FROM` command, which will cause earthly to execute all commands under the target directly
on the host system, rather than inside a container. Commands within a `LOCALLY` target will never be cached.
This feature should be used with caution as locally run commands have no guarantee they will behave the same on different systems.

`LOCALLY` defined targets only support a subset of commands (along with a subset of their flags): `RUN`, `RUN --push`, `SAVE ARTIFACT`, and `COPY`.

`RUN` commands have access to the environment variables which are exposed to the `earthly` command; however, the commands
are executed within a working directory which is set to the location of the referenced Earthfile and not where the `earthly` command is run from.

For example, the following Earthfile will display the current user, hostname, and directory where the Earthfile is stored:

```Dockerfile
whoami:
    LOCALLY
    RUN echo "I am currently running under $USER on $(hostname) under $(pwd)"
```

{% hint style='info' %}
##### Note
In Earthly, outputting images and artifacts locally takes place only at the end of a successful build. In order to use such images or artifacts in `LOCALLY` targets, they need to be referenced correctly.

For images, use the `--load` option under `WITH DOCKER`:

```Dockerfile
my-image:
    FROM alpine 3.13
    ...
    SAVE IMAGE my-example-image

a-locally-example:
    LOCALLY
    WITH DOCKER --load=+my-image
        RUN docker run --rm my-example-image
    END
```

Do NOT use `BUILD` for using images in `LOCALLY` targets:

```Dockerfile
# INCORRECT - do not use!
my-image:
    FROM alpine 3.13
    ...
    SAVE IMAGE my-example-image

a-locally-example:
    LOCALLY
    BUILD +my-image
    # The image will not be available here because the local export of the
    # image only takes place at the end of an entire successful build.
    RUN docker run --rm my-example-image
```

For artifacts, use `COPY`, the same way you would in a regular target:

```Dockerfile
my-artifact:
    FROM alpine 3.13
    ...
    SAVE ARTIFACT ./my-example-artifact

a-locally-example:
    LOCALLY
    COPY +my-artifact/my-example-artifact ./
    RUN cat ./my-example-artifact
```

Do NOT use `SAVE ARTIFACT ... AS LOCAL` and `BUILD` for referencing artifacts in `LOCALLY` targets:

```Dockerfile
# INCORRECT - do not use!
my-artifact:
    FROM alpine 3.13
    ...
    SAVE ARTIFACT ./my-example-artifact AS LOCAL ./my-example-artifact

a-locally-example:
    LOCALLY
    BUILD +my-artifact
    # The artifact will not be available here because the local export of the
    # artifact only takes place at the end of an entire successful build.
    RUN cat ./my-example-artifact
```
{% endhint %}

