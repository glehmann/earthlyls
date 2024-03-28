## SAVE IMAGE

#### Synopsis

* `SAVE IMAGE [--cache-from=<cache-image>] [--push] <image-name>...` (output form)
* `SAVE IMAGE --cache-hint` (cache hint form)

#### Description

In the *output form*, the command `SAVE IMAGE` marks the current build environment as the image of the target and assigns one or more output image names.

In the *cache hint form*, it instructs Earthly that the current target should be included as part of the explicit cache. For more information see the [remote caching guide](../caching/caching-via-registry.md).

{% hint style='info' %}
##### Assigning multiple image names

The `SAVE IMAGE` command allows you to assign more than one image name:

```Dockerfile
SAVE IMAGE my-image:latest my-image:1.0.0 my-example-registry.com/another-image:latest
```

Or

```Dockerfile
SAVE IMAGE my-image:latest
SAVE IMAGE my-image:1.0.0
SAVE IMAGE my-example-registry.com/another-image:latest
```
{% endhint %}

{% hint style='danger' %}
##### Important

As of [`VERSION 0.6`](#version), images are only saved [if they are connected to the initial target through a chain of `BUILD` commands](#what-is-being-output-and-pushed).

{% endhint %}

#### Options

##### `--push`

The `--push` options marks the image to be pushed to an external registry after it has been loaded within the docker daemon available on the host.

If inline caching is enabled, the `--push` option also instructs Earthly to use the specified image names as cache sources.

The actual push is not executed by default. Add the `--push` flag to the earthly invocation to enable pushing. For example

```bash
earthly --push +docker-image
```

##### `--cache-from=<cache-image>`

Adds additional cache sources to be used when `--use-inline-cache` is enabled. For more information see the [remote caching guide](../caching/caching-via-registry.md).

##### `--cache-hint`

Instructs Earthly that the current target should be included as part of the explicit cache. For more information see the [remote caching guide](../caching/caching-via-registry.md).

##### `--no-manifest-list`

Instructs Earthly to not create a manifest list for the image. This may be useful on platforms that do not support multi-platform images (for example, AWS Lambda), and the image produced needs to be of a different platform than the default one.

