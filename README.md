# Before anything else

Run `docker build -t beer_lambda .` to build the docker build container.

This is copied from [softprop/lambda-rust](https://github.com/softprops/lambda-rust) but I wanted to customize the build script and use Rust 1.32.0.

This uses the lambci build-provided docker image to provide the environment to build the binary in so there's no complicated building with different toolchains and downloading cross-compiler steps.

Also note that I'm running this with Docker for Mac: I had to change the DNS settings for Docker... It was set to 10.4.4.10, I changed the settings (Docker icon in the toolbar -> Preferences -> Daemon -> Advanced -> DNS in the JSON) to 8.8.8.8 and 8.8.4.4 (OpenDNS). Then I was able to get internet access and build the image as well as access the internet for Cargo Crates.

# To build the image

Run `./build.sh`. This mounts the current directory as /code as well as some .cargo directories and then builds the code quite simply with cargo. It runs from the custom-docker-build.sh script which is basically softprop's build script, but I plan to package multiple binaries with one repo since I don't want multiple repos for multiple lambdas that will do slightly different things.

# To test

Run `./test.sh`

This runs the handler within the beer\_lambda image with a test JSON blob provided.
