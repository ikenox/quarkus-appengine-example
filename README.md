# Quarkus Sample App on Google App Engine Java 11

## Requirements

- Docker
- JDK 1.8+
    - `$JAVA_HOME` needs to be set correctly
- Setup GraalVM (If you want to build native-image)
    - See: https://quarkus.io/guides/building-native-image-guide
    - After GraalVM installation, you also need to install native-image tool by `gu install native-image`
        - You may need to add `$GRAALVM_HOME/bin` into `$PATH`

## Run on local

- JVM

    ```sh
    ./mvnw package
    java -jar ./target/quarkus-appengine-example-1.0-SNAPSHOT-runner.jar
    ```

- Native image

    ```sh
    ./mvnw package -Pnative
    ./target/quarkus-appengine-example-1.0-SNAPSHOT-runner
    ```

## Deploy to App Engine Java 11

- JVM

    ```sh
    ./mvnw package
    gcloud app deploy app-jvm.yaml --project=your-gcp-project-id --version=jvm
    ```

- Native image
    
    ```sh
    ./mvnw package -Pnative -Dnative-image.docker-build=true
    gcloud app deploy app-native.yaml --project=your-gcp-project-id --version=native-image
    ```

- Rust

    ```sh
    docker run \
         --rm \
         --interactive \
         --tty \
         --volume (pwd):/opt/volume \
         --workdir /opt/volume \
         amd64/rust cargo build --release
    gcloud app deploy app-rust.yaml --project=your-gcp-project-id --version=rust-sample
    ```
