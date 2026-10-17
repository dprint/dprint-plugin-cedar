# dprint-plugin-cedar

[![npm version](https://img.shields.io/npm/v/@dprint/cedar.svg)](https://www.npmjs.com/package/@dprint/cedar) [![CI](https://github.com/dprint/dprint-plugin-cedar/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-cedar/actions?query=workflow%3ACI)

Adapter for [cedar-policy-formatter](https://github.com/cedar-policy/cedar) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [Cedar](https://www.cedarpolicy.com) policy files (`.cedar`).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add cedar
```

Note: You do not need the Cedar CLI installed globally as dprint will run cedar-policy-formatter from the .wasm file in a sandboxed environment.

## Configuration

To add configuration, specify a `"cedar"` key in your dprint.json:

```jsonc
{
  "cedar": {
    "lineWidth": 80,
    "indentWidth": 2,
  },
  "plugins": [
    // ...etc...
  ],
}
```

For an overview of the config, see https://dprint.dev/plugins/cedar/config/

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/cedar)

## Versioning

This repo automatically upgrades to the latest version of cedar-policy-formatter once a day. You can check which version of cedar-policy-formatter is being used by looking at the `cedar-policy-formatter` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-cedar/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of cedar-policy-formatter.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
