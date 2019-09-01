# Quarku on Google App Engine Java 11

## Native

- Download GraalVM and set $GRAALVM_HOME
- set GRAALVM_HOME ~/Development/graalvm-ce-19.2.0/Contents/Home
- set -x PATH ~/Development/graalvm-ce-19.2.0/Contents/Home/bin $PATH
- gu install native-image
- run on mac
    - ./mvnw package -Pnative
    - ./target/quarkusexample-1.0-SNAPSHOT-runner
- build native image for GAE/Java11
    - ./mvnw package -Pnative -Dnative-image.docker-build=true
    - gcloud app deploy app-native.yaml --project=ikenox-sunrise --version=native-image

## Normal (run on JVM)
./mvnw package
- gcloud app deploy app-jvm.yaml --project=ikenox-sunrise --version=jvm
