unzip -o target/lambda/release/beer_lambda.zip -d /tmp/lambda && docker run --rm -v /tmp/lambda:/var/task --env-file ~/.aws/docker_credentials lambci/lambda:provided handler '{"text":"Ben"}'
