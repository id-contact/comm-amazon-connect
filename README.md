# Amazon Connect communication plugin for ID Contact

This respository is a part of the ID Contact ecosystem. It contains a communication plugin used to hook up the
[Amazon Connect](https://aws.amazon.com/connect/) contact center to the ID Contact infrastructure.

This implementation is intended as a reference for other telephony plugins. It aims to keep the
telephony software as agnostic as possible to ID Contact, by implementing only the minimally required interaction.

The plugin consists of 3 parts, only one of which is actually specific to Amazon Connect:

- The plugin backend, which is responsible for talking to the rest of the ID Contact ecosystem
- The attribute display, which is responsible for displaying attributes in an iframe embedded in the interface for the service employee
- The modifications to the Amazon Connect flow, which are responsible for capturing the DTMF code and instructing the iframe to show the correct session data

The plugin backend is written in [Rust](https://www.rust-lang.org/), the web interface is created using Typescript and React.

## Getting started

To build and run this plugin backend, do:
```
ROCKET_CONFIG=config.sample.toml cargo run
```

The attribute UI can be build by:
```
cd attribute-ui
yarn
yarn run build
```

You will need a webserver (like NGINX) to serve static files and perform the necessary routing. 

## Further reading

Complete documentation for this plugin can be found in [the general ID Contact documentation](https://docs.idcontact.nl)