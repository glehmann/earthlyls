## PROJECT

#### Synopsis

* `PROJECT <org-name>/<project-name>`

#### Description

The command `PROJECT` marks the current Earthfile as being part of the project belonging to the [Earthly organization](https://docs.earthly.dev/earthly-cloud/overview) `<org-name>` and the project `<project-name>`. The project is used by Earthly to retrieve [cloud-based secrets](../cloud/cloud-secrets.md) and build logs belonging to the project.

The `PROJECT` command can only be used in the `base` recipe and it applies to the entire Earthfile. The `PROJECT` command can never contain any `ARG`s that need expanding.

