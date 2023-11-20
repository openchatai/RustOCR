# Rust OCR Microservice

## Overview

This project is a Rust-based microservice designed for extracting text from scanned documents using Optical Character Recognition (OCR) technology. The microservice provides a simple and efficient way to upload scanned documents and retrieve the extracted text.

## Features

- **Text Extraction**: Utilize OCR capabilities to extract text from uploaded scanned documents.

## Usage
1. Ensure you have Rust installed on your system.

2. Clone the repository:

    ```bash
    git clone https://github.com/openchatai/rust_ocr
    ```

3. Navigate to the project directory:

    ```bash
    cd rust_ocr
    ```

4. Build the project:

    ```bash
    cargo build --release
    ```

5. Run the microservice:

    ```bash
    ./target/release/rust_ocr
    ```

   The microservice will be running on `http://localhost:8000`.

   **Docker Image:**

   The latest build will appear for arm by default whenever code is pushed to the GitHub repo. For other architectures, use the edge build tag or build from source. Contributions are welcome.

   [Docker Hub - Rust OCR Microservice](https://hub.docker.com/repository/docker/codebanesr/rust_ocr/tags?page=1&ordering=last_updated)

## Using Prebuilt Docker Images

Prebuilt Docker images for the Rust OCR Microservice are available on Docker Hub. You can choose the appropriate image for your architecture from the following link:

- [Docker Hub - Rust OCR](https://hub.docker.com/repository/docker/codebanesr/rust_ocr/tags?page=1&ordering=last_updated)

Once you have pulled the Docker image, you can run the Rust OCR Microservice using the following command:

```bash
docker run -p 8000:8000 codebanesr/rust_ocr:your_tag
```

Replace `your_tag` with the specific tag of the Docker image you want to use.

## Using Docker Compose

```yaml
version: '3'

services:
  rust_ocr:
    image: codebanesr/rust_ocr:edge
    ports:
      - "8000:8000"
```

### Text Extraction

Retrieve the extracted text using the following `curl` command:

```bash
curl --location 'http://localhost:8000/api/file/ocr' \
--form 'file=@"/Users/shanurrahman/Downloads/sample_scannable.png"'
```
Replace `sample_scannable.png` with your file path.

## Contributors

- [Shanur Rahman](https://github.com/codebanesr)
- [G](https://github.com/gharbat)
- [Ahmad Hassan](https://github.com/ah7255703)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
