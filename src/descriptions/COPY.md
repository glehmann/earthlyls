## COPY

#### Synopsis

* `COPY [options...] <src>... <dest>` (classical form)
* `COPY [options...] <src-artifact>... <dest>` (artifact form)
* `COPY [options...] (<src-artifact> --<build-arg-key>=<build-arg-value>...) <dest>` (artifact form with build args)

#### Description

The command `COPY` allows copying of files and directories between different contexts.

The command may take a couple of possible forms. In the *classical form*, `COPY` copies files and directories from the build context into the build environment - in this form, it works similarly to the [Dockerfile `COPY` command](https://docs.docker.com/engine/reference/builder/#copy). In the *artifact form*, `COPY` copies files or directories (also known as "artifacts" in this context) from the artifact environment of other build targets into the build environment of the current target. Either form allows the use of wildcards for the sources.

The parameter `<src-artifact>` is an [artifact reference](../guides/importing.md#artifact-reference) and is generally of the form `<target-ref>/<artifact-path>`, where `<target-ref>` is the reference to the target which needs to be built in order to yield the artifact and `<artifact-path>` is the path within the artifact environment of the target, where the file or directory is located. The `<artifact-path>` may also be a wildcard.

The `COPY` command does not mark any saved images or artifacts of the referenced target for output, nor does it mark any push commands of the referenced target for pushing. For that, please use [`BUILD`](#build).

Multiple `COPY` commands issued one after the other will build the referenced targets in parallel, if the targets don't depend on each other. The resulting artifacts will then be copied sequentially in the order in which the `COPY` commands were issued.

The classical form of the `COPY` command differs from Dockerfiles in three cases:

* URL sources are not yet supported.
* Absolute paths are not supported - sources in the current directory cannot be referenced with a leading `/`
* The Earthly `COPY` is a classical `COPY --link`. It uses layer merging for the copy operations.

{% hint style='info' %}
##### Note
To prevent Earthly from copying unwanted files, you may specify file patterns to be excluded from the build context using an [`.earthlyignore`](./earthlyignore.md) file. This file has the same syntax as a [`.dockerignore` file](https://docs.docker.com/engine/reference/builder/#dockerignore-file).
{% endhint %}

#### Options

##### `--dir`

The option `--dir` changes the behavior of the `COPY` command to copy the directories themselves, rather than the contents of the directories. It allows the command to behave similarly to a `cp -r` operation on a unix system. This allows the enumeration of several directories to be copied over on a single line (and thus, within a single layer). For example, the following two are equivalent with respect to what is being copied in the end (but not equivalent with respect to the number of layers used).

```Dockerfile
COPY dir1 dir1
COPY dir2 dir2
COPY dir3 dir3
```

```Dockerfile
COPY --dir dir1 dir2 dir3 ./
```

If the directories were copied without the use of `--dir`, then their contents would be merged into the destination.

##### `--<build-arg-key>=<build-arg-value>`

Sets a value override of `<build-arg-value>` for the build arg identified by `<build-arg-key>`, when building the target containing the mentioned artifact. See also [BUILD](#build) for more details about the build arg options.

Note that build args and the artifact references they apply to need to be surrounded by parenthesis:

```Dockerfile
COPY (+target1/artifact --arg1=foo --arg2=bar) ./dest/path
```

##### `--keep-ts`

Instructs Earthly to not overwrite the file creation timestamps with a constant.

##### `--keep-own`

Instructs Earthly to keep file ownership information. This applies only to the *artifact form* and has no effect otherwise.

##### `--chmod <octal-format>`

Instructs Earthly to change the file permissions of the copied files. The `<chmod>` needs to be in octal format, e.g. `--chmod 0755` or `--chmod 755`.

{% hint style='info' %}
Note that you must include the flag in the corresponding `SAVE ARTIFACT --keep-own ...` command, if using *artifact form*.
{% endhint %}

##### `--if-exists`

Only copy source if it exists; if it does not exist, earthly will simply ignore the COPY command and won't treat any missing sources as failures.

##### `--symlink-no-follow`

Allows copying a symbolic link from another target; it has no effect when copying files from the host.
The option must be used in both the `COPY` and `SAVE ARTIFACT` commands; for example:

```Dockerfile
producer:
    RUN ln -s nonexistentfile symlink
    SAVE ARTIFACT --symlink-no-follow symlink

consumer:
    COPY --symlink-no-follow +producer/symlink
```

##### `--from`

Although this option is present in classical Dockerfile syntax, it is not supported by Earthfiles. You may instead use a combination of `SAVE ARTIFACT` and `COPY` *artifact form* commands to achieve similar effects. For example, the following Dockerfile

```Dockerfile
# Dockerfile
COPY --from=some-image /path/to/some-file.txt ./
```

... would be equivalent to `final-target` in the following Earthfile

```Dockerfile
# Earthfile
intermediate:
    FROM some-image
    SAVE ARTIFACT /path/to/some-file.txt

final-target:
    COPY +intermediate/some-file.txt ./
```

##### `--platform <platform>`

In *artifact form*, it specifies the platform to build the artifact on.

For more information see the [multi-platform guide](../guides/multi-platform.md).

##### `--allow-privileged`

Same as [`FROM --allow-privileged`](#allow-privileged).

##### `--pass-args`

Same as [`FROM --pass-args`](#pass-args).

##### `--build-arg <key>=<value>` (**deprecated**)

The option `--build-arg` is deprecated. Use `--<build-arg-key>=<build-arg-value>` instead.

#### Examples

Assuming the following directory tree, of a folder named `test`:

```
test
  └── file
```

Here is how the following copy commands will behave:

```
# Copies the contents of the test directory.
# To access the file, it would be found at ./file
COPY test .

# Also copies the contents of the test directory.
# To access the file, it would be found at ./file
COPY test/* .

# Copies the whole test folder.
# To access the file, it would be found at ./test/file
COPY --dir test .
```

One can also copy from other Earthfile targets:

```
FROM alpine:3.18
dummy-target:
    RUN echo aGVsbG8= > encoded-data
    SAVE ARTIFACT encoded-data
example:
    COPY +dummy-target/encoded-data .
    RUN cat encoded-data | base64 -d
```

Parentheses are required when passing build-args:

```
FROM alpine:3.18
RUN apk add coreutils # required for base32 binary
dummy-target:
    ARG encoder="base64"
    RUN echo hello | $encoder > encoded-data
    SAVE ARTIFACT encoded-data
example:
    COPY ( +dummy-target/encoded-data --encoder=base32 ) .
    RUN cat encoded-data | base32 -d
```

For detailed examples demonstrating how other scenarios may function, please see our [test suite](https://github.com/earthly/earthly/blob/main/tests/copy.earth).

