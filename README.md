# Quarkus on Google App Engine Java 11

## Requirements

- Docker
- Setup GraalVM (If you want to compile to native-image)
    - See: https://quarkus.io/guides/building-native-image-guide
    - After GraalVM installation, you also need to install native-image tool by `gu install native-image`
        - You may need to set `PATH` to `$GRAALVM_HOME/bin`

## Run on local

- JVM

    ```sh
    ./mvnw package
    ./target/quarkus-appengine-example-1.0-SNAPSHOT-runner.jar
    ```

- Native image

    ```sh
    ./mvnw package -Pnative
    ./target/quarkus-appengine-example-1.0-SNAPSHOT-runner
    ```

## Deploy to App Engine Java 11

### Normal (run on JVM)

```sh
./mvnw package
gcloud app deploy app-jvm.yaml --project=ikenox-sunrise --version=jvm
```

### Native image
    
```sh
./mvnw package -Pnative -Dnative-image.docker-build=true
gcloud app deploy app-native.yaml --project=ikenox-sunrise --version=native-image
```

