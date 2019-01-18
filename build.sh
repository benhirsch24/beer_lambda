docker run --rm -v ${PWD}:/code -v ${HOME}/.cargo/registry:/root/.cargo/registry -v ${HOME}/.cargo/git:/root/.cargo/git --entrypoint=/code/custom-docker-build.sh beer_lambda
