## CACHE

#### Synopsis

* ```
  CACHE [--sharing <sharing-mode>] [--chmod <octal-format>] [--id <cache-id>] [--persist] <mountpoint>
  ```

#### Description

The `CACHE` command creates a cache mountpoint at `<mountpoint>` in the build environment. The cache mountpoint is a directory which is shared between the instances of the same build target. The contents of the cache mountpoint are preserved between builds, and can be used to share data across builds.

#### Options

##### `--sharing <sharing-mode>`

The sharing mode for the cache mount, from one of the following:

* `locked` (default) - the cache mount is locked for the duration of the execution, other concurrent builds will wait for the lock to be released.
* `shared` - the cache mount is shared between all concurrent builds.
* `private` - if another concurrent build attempts to use the cache, a new (empty) cache will be created for the concurrent build.

##### `--chmod <octal-format>`

The permission of the mounted folder, in octal format (the same format the chmod unix command line expects).
Default `--chmod 0644`


##### `--id <cache-id>`

The cache ID for a global cache volume to be used across other targets or Earthfiles.

##### `--persist`

Make a copy of the cache available to any children that inherit from this target, by copying the contents of the cache to the child image.

{% hint style='warning' %}
Caches were persisted by default in version 0.7, which led to bloated images being pushed to registries. Version 0.8 changed the default behavior
to prevent copying the contents to children targets unless explicitly enabled by the newly added `--persist` flag.
{% endhint %}

