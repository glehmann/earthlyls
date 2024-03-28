## PIPELINE (**deprecated**)

{% hint style='info' %}
##### Note
The `PIPELINE` command is in beta status and is only useful for Earthly CI.
{% endhint %}

#### Synopsis

* `PIPELINE [--push]`

#### Description

The `PIPELINE` command is used to declare that the current target is an Earthly CI pipeline. The `PIPELINE` command must be the first command in the target.

To use a `PIPELINE`, you must also declare a `PROJECT` in the Earthfile. This `PROJECT` must match the name of the CI's project that references the git repository

A pipeline is a target that is executed by Earthly CI when a certain trigger is activated. Triggers can be declared via the `TRIGGER` command. Pipeline targets allow only the commands `TRIGGER`, `ARG` and `BUILD`. Other commands may be used indirectly in other targets that can be then referenced by `BUILD`.

Pipeline targets are always executed with no outputs, in strict mode.

{% hint style='info' %}
##### Note
Pipelines and their definitions, including their triggers must be merged into the primary branch (which, unless overridden, is the default branch on GitHub) in order for the triggers to take effect.
{% endhint %}

#### Example

The following example shows a simple pipeline called `my-pipeline`, which is triggered on either a push to the `main` branch, or a pull request against the `main` branch. The pipeline executes the target `my-build`, which simply prints `Hello world`.

```Earthfile
VERSION 0.8
PROJECT my-org/my-project

FROM alpine:3.18

my-pipeline:
  PIPELINE
  TRIGGER push main
  TRIGGER pr main
  BUILD +my-build

my-build:
  RUN echo Hello world
```

#### Options

##### `--push`

Indicates that the targets referenced by this pipeline will be called in push-mode. `SAVE IMAGE --push` commands will trigger pushes to the remote registry, and `RUN --push` commands will execute.

