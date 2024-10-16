# TermiMail

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Edition%202021-blue.svg?logo=rust)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](https://github.com/nicolasleal570/termimail/releases)

Generate temporary email accounts directly from your terminal with ease. **TermiMail** is a Rust-based CLI tool designed to streamline the way developers and tech enthusiasts manage disposable emails right from the command line.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
  - [MacOS](#macos)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

- **Generate Temporary Email Accounts:** Create disposable email addresses instantly.
- **Fetch Messages:** Retrieve emails from your temporary inbox.
- **View Account Details:** Access your current account information at any time.
- **Delete Accounts:** Clean up your temporary accounts when they're no longer needed.
- **Cross-Platform:** Works on macOS (Windows and Linux coming soon).
- **Privacy-Focused:** Uses the [mail.tm](https://docs.mail.tm/) API for secure and anonymous email handling.

## Installation

#### MacOS
```console
brew tap nicolasleal570/termimail
brew install termimail
```

Windows and Linux support are coming soon... 

## Usage 
TermiMail provides a simple command-line interface to manage temporary emails.

### Generate a new email account
```console
termimail g
```

### View current account details
```console
termimail me
```

### Fetch messages from inbox
```console
termimail m
```

### Delete current account
```console
termimail d
```

## Contributing
Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch:
```console 
git checkout -b feature/your-feature-name
```
3. Make your changes.
4. Commit your changes:
```console 
git commit -m "Add your message"
```
5. Push to your branch:
```console 
git push origin feature/your-feature-name
```
6. Open a Pull Request.

Please ensure your code adheres to the existing style and includes tests where appropriate.

# License
This project is licensed under the MIT License. See the LICENSE file for details.

# Acknowledgments
- [mail.tm](https://mail.tm/en/) API for providing a reliable temporary email service.
- The Rust community for their invaluable resources and support.
- [Mailsy](https://github.com/BalliAsghar/Mailsy) for the inspiration.