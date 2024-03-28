## HEALTHCHECK (same as Dockerfile HEALTHCHECK)

#### Synopsis

* `HEALTHCHECK NONE` (disable health checking)
* `HEALTHCHECK [--interval=DURATION] [--timeout=DURATION] [--start-period=DURATION] [--retries=N] [--start-interval=DURATION] CMD command arg1 arg2` (check container health by running command inside the container)

#### Description

The `HEALTHCHECK` command tells Docker how to test a container to check that it is still working. It works the same way as the [Dockerfile `HEALTHCHECK` command](https://docs.docker.com/engine/reference/builder/#healthcheck), with the only exception that the exec form of this command is not yet supported.

#### Options

##### `--interval=DURATION`

Sets the time interval between health checks. Defaults to `30s`.

##### `--timeout=DURATION`

Sets the timeout for a single run before it is considered as failed. Defaults to `30s`.

##### `--start-period=DURATION`

Sets an initialization time period in which failures are not counted towards the maximum number of retries. Defaults to `0s`.

##### `--retries=N`

Sets the number of retries before a container is considered `unhealthy`. Defaults to `3`.

##### `--start-interval=DURATION`

Sets the time interval between health checks during the start period. Defaults to `5s`.

