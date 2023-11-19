# Rust OCR Microservice

## Overview

This project is a Rust-based microservice designed for extracting text from scanned documents using Optical Character Recognition (OCR) technology. The microservice provides a simple and efficient way to upload scanned documents and retrieve the extracted text.

## Features

- **File Upload**: Easily upload scanned documents in various formats for text extraction.

- **Text Extraction**: Utilize OCR capabilities to extract text from uploaded scanned documents.

## Installation

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

## Usage

### File Upload

Use the following `curl` command to upload a scanned document:

```bash
curl --location 'http://localhost:8000/api/file/upload' \
--form 'file=@"/path/to/your/scanned/document.png"'
```

### Text Extraction

Retrieve the extracted text from a previously uploaded document using the following `curl` command:

```bash
curl --location 'http://localhost:8000/api/ocr?filename=your_document_filename.png'
```

Replace `your_document_filename.png` with the actual filename of the document you uploaded.

## Contributors

- [Shanur Rahman](https://github.com/codebanesr)
- [G](https://github.com/gharbat)
- [Ahmad Hassan](https://github.com/ah7255703)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
