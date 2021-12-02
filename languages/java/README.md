# Tangram for Java

- [Watch the Video](https://www.tangram.dev)
- [Read the Docs](https://www.tangram.dev/docs)

The Tangram Java library makes it easy to make predictions with your Tangram machine learning model from Java.
## Build

To build the Tangram JNI native library run

```
$ cd languages/java && cargo build
```


## Running
After building the native JNI library you can run programs with the following command

```
$ JAVA_LIBRARY_PATH=/path/to/tangram/target/debug java BasicTangramExample
```

For more information, [read the docs](https://www.tangram.dev/docs).

## Examples

The source for this package contains a number of examples in the `examples` directory. Each example has a `README.md` explaining how to run it.
