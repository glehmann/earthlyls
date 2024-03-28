## SHELL (not supported)

The classical [`SHELL` Dockerfile command](https://docs.docker.com/engine/reference/builder/#shell) is not yet supported. Use the *exec form* of `RUN`, `ENTRYPOINT` and `CMD` instead and prepend a different shell.

