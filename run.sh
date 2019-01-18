unzip -o target/lambda/release/beer_lambda.zip -d /tmp/lambda && docker run --rm -v /tmp/lambda:/var/task lambci/lambda:provided handler '{"text":"Ben"}'
