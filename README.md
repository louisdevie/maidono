# maidono

Linux service to listen to GitHub webhooks

## Installation

### On 64-bit Linux with systemd

Download the latest binary release from [GitHub](https://github.com/louisdevie/maidono/releases), unpack it and run the
`install.sh` script as root.

### On other systems

To build maidono from source, you'll need Rust 1.78.0 or higher (see the
[installation instructions](https://www.rust-lang.org/tools/install)).

To build the server and the CLI tool :

```shell
cd maidctl
cargo build -r
```

```shell
cd maidono
cargo build -r
```

The built executables can then be found at `maidctl/target/release/maidctl` and `maidono/target/release/maidctl`. You
can copy them somewhere on the PATH (like `/usr/bin`).

Maidono will use `/etc/maidono` as its config directory. To change it, you can to edit the file
`core/src/utils/path.rs`. You'll need to rebuild *both* executables after modifying this file.

The file `/etc/maidono/enabled` need to exist for the server and the CLI tool to work. Just create an empty file at this
path.

## Actions config

### Basic example

A maidono *action* is a webhook endpoint. To create an action, add a `something.yml` file under the
`/etc/maidono/actions` directory (the extension doesn't actually matter, any file in this directory will be read as a
config file). Files here are called *groups*. If you run `maidctl list` you should see the
name of the group you just created.

Then add an action to this file :

```yaml
- name: Hello world
  on: /hello-world
  run: echo 'Hello, world!'
```

If you (re)start the server, it should log "Hello, world!" each time a POST request is made to
`http://localhost:4471/hello-world`.

### Usage with GitHub

Currently, only GitHub webhooks can be authenticated. Here is an example of an action for GitHub events :

```yaml
- name: my github action
  on: /gh-test
  from: github
  secret: <your authentication secret here>
  run: echo 'Hello, world!'
```

When adding the webhook in GitHub, put `https://your-server-url/gh-test` under **Payload URL**, `application/json` under
**Content type** and the same secret under **Secret**.

## License

This project is distributed under the [MIT license](/LICENSE).