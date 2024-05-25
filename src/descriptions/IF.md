## IF

#### Synopsis

* ```
  IF [<condition-options>...] <condition>
    <if-block>
  END
  ```
* ```
  IF [<condition-options>...] <condition>
    <if-block>
  ELSE
    <else-block>
  END
  ```
* ```
  IF [<condition-options>...] <condition>
    <if-block>
  ELSE IF [<condition-options>...] <condition>
    <else-if-block>
  ...
  ELSE
    <else-block>
  END
  ```

#### Description

The `IF` clause can perform varying commands depending on the outcome of one or more conditions. The expression passed as part of `<condition>` is evaluated by running it in the build environment. If the exit code of the expression is zero, then the block of that condition is executed. Otherwise, the control continues to the next `ELSE IF` condition (if any), or if no condition returns a non-zero exit code, the control continues to executing the `<else-block>`, if one is provided.

#### Examples

A very common pattern is to use the POSIX shell `[ ... ]` conditions. For example the following marks port `8080` as exposed if the file `./foo` exists.

```Dockerfile
IF [ -f ./foo ]
  EXPOSE 8080
END
```

It is also possible to call other commands, which can be useful for more comparisons such as semantic versioning. For example:

```Dockerfile
VERSION 0.8

test:
  FROM python:3
  RUN pip3 install semver

  # The following python script requires two arguments (v1 and v2)
  # and will return an exit code of 0 when v1 is semantically greater than v2
  # or an exit code of 1 in all other cases.
  RUN echo "#!/usr/bin/env python3
import sys
import semver
v1 = sys.argv[1]
v2 = sys.argv[2]
if semver.compare(v1, v2) > 0:
  sys.exit(0)
sys.exit(1)
  " > ./semver-gt && chmod +x semver-gt

  # Define two different versions
  ARG A="0.3.2"
  ARG B="0.10.1"

  # and compare them
  IF ./semver-gt "$A" "$B"
    RUN echo "A ($A) is semantically greater than B ($B)"
  ELSE
    RUN echo "A ($A) is NOT semantically greater than B ($B)"
  END
```

{% hint style='info' %}
##### Note
Performing a condition requires that a `FROM` (or a from-like command, such as `LOCALLY`) has been issued before the condition itself.

For example, the following is NOT a valid Earthfile.

```Dockerfile
# NOT A VALID EARTHFILE.
ARG base=alpine
IF [ "$base" = "alpine" ]
    FROM alpine:3.18
ELSE
    FROM ubuntu:20.04
END
```

The reason this is invalid is because the `IF` condition is actually running the `/usr/bin/[` executable to test if the condition is true or false, and therefore requires that a valid build environment has been initialized.

Here is how this might be fixed.

```Dockerfile
ARG base=alpine
FROM busybox
IF [ "$base" = "alpine" ]
    FROM alpine:3.18
ELSE
    FROM ubuntu:20.04
END
```

By initializing the build environment with `FROM busybox`, the `IF` condition can execute on top of the `busybox` image.
{% endhint %}

{% hint style='danger' %}
##### Important
Changes to the filesystem in any of the conditions are not preserved. If a file is created as part of a condition, then that file will not be present in the build environment for any subsequent commands.
{% endhint %}

#### Options

##### `--privileged`

Same as [`RUN --privileged`](#privileged).

##### `--ssh`

Same as [`RUN --ssh`](#ssh).

##### `--no-cache`

Same as [`RUN --no-cache`](#no-cache).

##### `--mount <mount-spec>`

Same as [`RUN --mount <mount-spec>`](#mount-less-than-mount-spec-greater-than).

##### `--secret <env-var>=<secret-id>`

Same as [`RUN --secret <env-var>=<secret-id>`](#secret-less-than-env-var-greater-than-less-than-secret-id-greater-than).

