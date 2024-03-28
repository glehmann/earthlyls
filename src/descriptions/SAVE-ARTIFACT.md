## SAVE ARTIFACT

#### Synopsis

* `SAVE ARTIFACT [--keep-ts] [--keep-own] [--if-exists] [--force] <src> [<artifact-dest-path>] [AS LOCAL <local-path>]`

#### Description

The command `SAVE ARTIFACT` copies a file, a directory, or a series of files and directories represented by a wildcard, from the build environment into the target's artifact environment.

If `AS LOCAL ...` is also specified, it additionally marks the artifact to be copied to the host at the location specified by `<local-path>`, once the build is deemed as successful. Note that local artifacts are only produced by targets that are run directly with `earthly`, or when invoked using [`BUILD`](#build).

If `<artifact-dest-path>` is not specified, it is inferred as `/`.

Files within the artifact environment are also known as "artifacts". Once a file has been copied into the artifact environment, it can be referenced in other places of the build (for example in a `COPY` command), using an [artifact reference](../guides/importing.md#artifact-reference).

{% hint style='info' %}
##### Hint
In order to inspect the contents of an artifacts environment, you can run

```bash
earthly --artifact +<target>/* ./output/
```

This command dumps the contents of the artifact environment of the target `+<target>` into a local directory called `output`, which can be inspected directly.
{% endhint %}

{% hint style='danger' %}
##### Important
Note that there is a distinction between a *directory artifact* and *file artifact* when it comes to local output. When saving an artifact locally, a directory artifact will **replace** the destination entirely, while a file (or set of files) artifact will be copied **into** the destination directory.

```Dockerfile
# This will wipe ./destination and replace it with the contents of the ./my-directory artifact.
SAVE ARTIFACT ./my-directory AS LOCAL ./destination
# This will merge the contents of ./my-directory into ./destination.
SAVE ARTIFACT ./my-directory/* AS LOCAL ./destination
```
{% endhint %}

{% hint style='danger' %}
##### Important

As of [`VERSION 0.6`](#version), local artifacts are only saved [if they are connected to the initial target through a chain of `BUILD` commands](#what-is-being-output-and-pushed).

{% endhint %}

#### Options

##### `--keep-ts`

Instructs Earthly to not overwrite the file creation timestamps with a constant.

##### `--keep-own`

Instructs Earthly to keep file ownership information.

##### `--if-exists`

Only save artifacts if they exists; if not, earthly will simply ignore the SAVE ARTIFACT command and won't treat any missing sources as failures.

##### `--symlink-no-follow`

Save the symbolic link rather than the contents of the symbolically linked file. Note that the same flag must also be used in the corresponding `COPY` command. For example:

```Dockerfile
producer:
    RUN ln -s nonexistentfile symlink
    SAVE ARTIFACT --symlink-no-follow symlink

consumer:
    COPY --symlink-no-follow +producer/symlink
```


##### `--force`

Force save operations which may be unsafe, such as writing to (or overwriting) a file or directory on the host filesystem located outside of the context of the directory containing the Earthfile.

#### Examples

Assuming the following directory tree, of a folder named `test`:

```
test
  └── file

```

Here is how the following `SAVE ARTIFACT ... AS LOCAL` commands will behave:

```
WORKDIR base
COPY test .

# This will copy the base folder into the output directory.
# You would find file at out-dot/base/file.
SAVE ARTIFACT . AS LOCAL out-dot/

# This will copy the contents of the base folder into the output directory.
# You would find sub-file at out-glob/file. Note the base directory is not in the output.
SAVE ARTIFACT ./* AS LOCAL out-glob/
```

For detailed examples demonstrating how other scenarios may function, please see our [test suite](https://github.com/earthly/earthly/blob/main/tests/file-copying.earth).

