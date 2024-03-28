## RUN

#### Synopsis

* `RUN [options...] [--] <command>` (shell form)
* `RUN [[options...], "<executable>", "<arg1>", "<arg2>", ...]` (exec form)

#### Description

The `RUN` command executes commands in the build environment of the current target, in a new layer. It works similarly to the [Dockerfile `RUN` command](https://docs.docker.com/engine/reference/builder/#run), with some added options.

The command allows for two possible forms. The *exec form* runs the command executable without the use of a shell. The *shell form* uses the default shell (`/bin/sh -c`) to interpret the command and execute it. In either form, you can use a `\` to continue a single `RUN` instruction onto the next line.

When the `--entrypoint` flag is used, the current image entrypoint is used to prepend the current command.

To avoid any ambiguity regarding whether an argument is a `RUN` flag option or part of the command, the delimiter `--` may be used to signal the parser that no more `RUN` flag options will follow.

#### Options

##### `--push`

Marks the command as a "push command". Push commands are never cached, thus they are executed on every applicable invocation of the build.

Push commands are not run by default. Add the `--push` flag to the `earthly` invocation to enable pushing. For example

```bash
earthly --push +deploy
```

Push commands were introduced to allow the user to define commands that have an effect external to the build. Good candidates for push commands are uploads of artifacts to artifactories, commands that make a change to an external environment, like a production or staging environment.

##### `--no-cache`

Force the command to run every time; ignoring the layer cache. Any commands following the invocation of `RUN --no-cache`, will also ignore the cache. If `--no-cache` is used as an option on the `RUN` statement within a `WITH DOCKER` statement, all commands after the `WITH DOCKER` will also ignore the cache.

{% hint style='danger' %}
##### Auto-skip
Note that `RUN --no-cache` commands may still be skipped by auto-skip. For more information see the [Caching in Earthfiles guide](../caching/caching-in-earthfiles.md#auto-skip).
{% endhint %}

##### `--entrypoint`

Prepends the currently defined entrypoint to the command.

This option is useful for replacing `docker run` in a traditional build environment. For example, a command like

```bash
docker run --rm -v "$(pwd):/data" cytopia/golint .
```

Might become the following in an Earthfile

```Dockerfile
FROM cytopia/goling
COPY . /data
RUN --entrypoint .
```

##### `--privileged`

Allows the command to use privileged capabilities.

Note that privileged mode is not enabled by default. In order to use this option, you need to additionally pass the flag `--allow-privileged` (or `-P`) to the `earthly` command. Example:

```bash
earthly --allow-privileged +some-target
```

##### `--secret <env-var>=<secret-id> | <secret-id>`

Makes available a secret, in the form of an env var (its name is defined by `<env-var>`), to the command being executed.
If you only specify `<secret-id>`, the name of the env var will be `<secret-id>` and its value the value of `<secret-id>`.

Here is an example that showcases both syntaxes:

```Dockerfile
release:
    RUN --push --secret GITHUB_TOKEN=GH_TOKEN github-release upload
release-short:
    RUN --push --secret GITHUB_TOKEN github-release upload
```

```bash
earthly --secret GH_TOKEN="the-actual-secret-token-value" +release
earthly --secret GITHUB_TOKEN="the-actual-secret-token-value" +release-short
```

An empty string is also allowed for `<secret-id>`, allowing for optional secrets, should it need to be disabled.

```Dockerfile
release:
    ARG SECRET_ID=GH_TOKEN
    RUN --push --secret GITHUB_TOKEN=$SECRET_ID github-release upload
release-short:
    ARG SECRET_ID=GITHUB_TOKEN
    RUN --push --secret $SECRET_ID github-release upload
```

```bash
earthly +release --SECRET_ID=""
earthly +release-short --SECRET_ID=""
```

It is also possible to mount a secret as a file with `RUN --mount type=secret,id=secret-id,target=/path/of/secret,chmod=0400`. See `--mount` below.

For more information on how to use secrets see the [Secrets guide](../guides/secrets.md). See also the [Cloud secrets guide](../cloud/cloud-secrets.md).

##### `--network=none`

Isolate the networking stack (and internet access) from the command.

##### `--ssh`

Allows a command to access the ssh authentication client running on the host via the socket which is referenced by the environment variable `SSH_AUTH_SOCK`.

Here is an example:

```Dockerfile
RUN mkdir -p ~/.ssh && \
    echo 'github.com ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQCj7ndNxQowgcQnjshcLrqPEiiphnt+VTTvDP6mHBL9j1aNUkY4Ue1gvwnGLVlOhGeYrnZaMgRK6+PKCUXaDbC7qtbW8gIkhL7aGCsOr/C56SJMy/BCZfxd1nWzAOxSDPgVsmerOBYfNqltV9/hWCqBywINIR+5dIg6JTJ72pcEpEjcYgXkE2YEFXV1JHnsKgbLWNlhScqb2UmyRkQyytRLtL+38TGxkxCflmO+5Z8CSSNY7GidjMIZ7Q4zMjA2n1nGrlTDkzwDCsw+wqFPGQA179cnfGWOWRVruj16z6XyvxvjJwbz0wQZ75XK5tKSb7FNyeIEs4TT4jk+S4dhPeAUC5y+bDYirYgM4GC7uEnztnZyaVWQ7B381AK4Qdrwt51ZqExKbQpTUNn+EjqoTwvqNj4kqx5QUCI0ThS/YkOxJCXmPUWZbhjpCg56i+2aB6CmK2JGhn57K5mj0MNdBXA4/WnwH6XoPWJzK5Nyu2zB3nAZp+S5hpQs+p1vN1/wsjk=' >> ~/.ssh/known_hosts && \
    echo 'gitlab.com ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQCsj2bNKTBSpIYDEGk9KxsGh3mySTRgMtXL583qmBpzeQ+jqCMRgBqB98u3z++J1sKlXHWfM9dyhSevkMwSbhoR8XIq/U0tCNyokEi/ueaBMCvbcTHhO7FcwzY92WK4Yt0aGROY5qX2UKSeOvuP4D6TPqKF1onrSzH9bx9XUf2lEdWT/ia1NEKjunUqu1xOB/StKDHMoX4/OKyIzuS0q/T1zOATthvasJFoPrAjkohTyaDUz2LN5JoH839hViyEG82yB+MjcFV5MU3N1l1QL3cVUCh93xSaua1N85qivl+siMkPGbO5xR/En4iEY6K2XPASUEMaieWVNTRCtJ4S8H+9' >> ~/.ssh/known_hosts
RUN --ssh git config --global url."git@github.com:".insteadOf "https://github.com/" && \
    go mod download
```

{% hint style='warning' %}
Note that `RUN --ssh` option is only used for creating a tunnel to the host's ssh-agent's socket (set via `$SSH_AUTH_SOCK`); it is **not** related to the git section of the earthly [configuration file](../earthly-config/earthly-config.md).
{% endhint %}

##### `--mount <mount-spec>`

Mounts a file or directory in the context of the build environment.

The `<mount-spec>` is defined as a series of comma-separated list of key-values. The following keys are allowed

| Key             | Description                                                                                                                                                                                                                  | Example                                 |
|-----------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------|
| `type`          | The type of the mount. Currently only `cache`, `tmpfs`, and `secret` are allowed.                                                                                                                                            | `type=cache`                            |
| `target`        | The target path for the mount.                                                                                                                                                                                               | `target=/var/lib/data`                  |
| `mode`, `chmod` | The permission of the mounted file, in octal format (the same format the chmod unix command line expects).                                                                                                                   | `chmod=0400`                            |
| `id`            | The cache ID for a global cache mount to be used across other targets or Earthfiles, when `type=cache`. The secret ID for the contents of the `target` file, when `type=secret`. | `id=my-shared-cache`, `id=my-password`  |
| `sharing`       | The sharing mode (`locked`, `shared`, `private`) for the cache mount, only applicable for `type=cache`.                                                                                                                      | `sharing=shared`                        |

For cache mounts, the sharing mode can be one of the following:

* `locked` (default) - the cache mount is locked for the duration of the execution, other concurrent builds will wait for the lock to be released.
* `shared` - the cache mount is shared between all concurrent builds.
* `private` - if another concurrent build attempts to use the cache, a new (empty) cache will be created for the concurrent build.

###### Examples

Persisting cache for a single `RUN` command, even when its dependencies change:

```Dockerfile
ENV GOCACHE=/go-cache
RUN --mount=type=cache,target=/go-cache go build main.go
```

{% hint style='warning' %}
Note that mounts cannot be shared between targets, nor can they be shared within the same target, if the build-args differ between invocations.
{% endhint %}

Mounting a secret as a file:

```Dockerfile
RUN --mount=type=secret,id=netrc,target=/root/.netrc curl https://example.earthly.dev/restricted/example-file-that-requires-auth > data
```

The contents of the secret `/root/.netrc` file can then be specified from the command line as:

```bash
earthly --secret netrc="machine example.earthly.dev login myusername password mypassword" +base
```

or by passing the contents of an existing file from the host filesystem:

```bash
earthly --secret-file netrc="$HOME/.netrc" +base
```


##### `--interactive` / `--interactive-keep`

Opens an interactive prompt during the target build. An interactive prompt must:

1. Be the last issued command in the target, with the exception of `SAVE IMAGE` commands. This also means that you cannot `FROM` a target containing a `RUN --interactive`.
2. Be the only `--interactive` target within the run.
3. Not be within a `LOCALLY`-designated target.

###### Examples:

Start an interactive python REPL:
```Dockerfile
python:
    FROM alpine:3.18
    RUN apk add python
    RUN --interactive python
```

Start `bash` to tweak an image by hand. Changes made will be included:
```Dockerfile
build:
    FROM alpine:3.18
    RUN apk add bash
    RUN --interactive-keep bash
```

