## VERSION

#### Synopsis

* `VERSION [options...] <version-number>`

#### Description

The command `VERSION` identifies which set of features to enable in Earthly while handling the corresponding Earthfile. Different `VERSION`s can be mixed together across different Earthfiles in the same project. Earthly handles a mix of versions gracefully, enabling or disabling features accordingly. This allows for gradual updates of `VERSION`s across large projects, without sacrificing build consistency.

The `VERSION` command is mandatory starting with Earthly 0.7. The `VERSION` command must be the first command in the Earthfile.

#### Options

Individual features may be enabled by setting the corresponding feature flag.
New features start off as experimental, which is why they are disabled by default.
Once a feature reaches maturity, it will be enabled by default under a new version number.

{% hint style='danger' %}
##### Important
Avoid using feature flags for critical workflows. You should only use feature flags for testing new experimental features. By using feature flags you are opting out of forwards/backwards compatibility guarantees. This means that running the same script in a different environment, with a different version of Earthly may result in a different behavior (i.e. it'll work on your machine, but may break the build for your colleagues or for the CI).
{% endhint %}

All features are described in [the version-specific features reference](./features.md).

