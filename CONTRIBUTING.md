# Contributing to Kmouse

Thank you for your interest in contributing to **Kmouse**! We appreciate your time and effort to help make this project better.

To ensure a smooth and productive collaboration, please follow these guidelines:

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Submitting Code](#submitting-code)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## Code of Conduct

We expect all contributors to adhere to our [Code of Conduct](https://www.contributor-covenant.org/). Please be respectful, inclusive, and constructive in all interactions.

---

## How to Contribute

### Reporting Bugs

If you encounter any bugs while using Kmouse, please follow these steps to report them:

1. **Check for existing issues**: Before opening a new bug report, check if the issue has already been reported.
2. **Open a new issue**: If the bug hasn't been reported, open a new issue with the following information:
   - A clear and concise title.
   - Steps to reproduce the issue.
   - What you expected to happen.
   - What actually happened.
   - Relevant logs or screenshots, if possible.

### Suggesting Enhancements

We love feedback and suggestions! If you have an idea to improve Kmouse:

1. **Check for existing feature requests**: If your idea has already been suggested, add your feedback to the existing issue.
2. **Create a new enhancement issue**: If your suggestion is unique, open a new issue with:
   - A descriptive title.
   - Why you think the feature is important or beneficial.
   - Any relevant details on how the feature should work.

### Submitting Code

We welcome code contributions! To submit code, follow these steps:

1. **Fork the repository**: Fork the repository to your GitHub account.
2. **Create a new branch**: Always create a new branch for your feature or fix.
   - Naming convention: `feature/{feature-name}` or `fix/{bug-name}`
3. **Make your changes**: Implement the fix or feature in your branch.
4. **Run tests**: Ensure your changes don't break existing functionality.
5. **Commit your changes**: Write a meaningful commit message explaining what you’ve done.
   - Follow the [commit message conventions](https://www.conventionalcommits.org/en/v1.0.0/) (e.g., `feat: add right-click support`).
6. **Push your branch**: Push your changes to your forked repository.

---

## Development Setup

To contribute to Kmouse, you'll need to set up the development environment. Follow these steps to get started:

### Prerequisites

- **Rust toolchain**: 1.70.0 or newer
- **X11 development libraries**: For Linux-based systems
- **Cargo**: Rust's package manager and build system

### Setting Up Locally

1. Clone the repository:

    ```bash
    git clone https://github.com/lovenangelo/kmouse.git
    cd kmouse
    ```

2. Install dependencies and build the project:

    ```bash
    cargo build --release
    ```

3. Run the project:

    ```bash
    cargo run
    ```

---

## Pull Request Process

1. **Open a pull request (PR)**: Once you’ve pushed your branch, open a pull request to the `main` branch of this repository.
2. **Describe your changes**: In the PR description, explain what your changes do and why they are necessary.
3. **Review process**: All pull requests will be reviewed by the maintainers before being merged. This may include code improvements or additional tests.
4. **Resolve feedback**: If the maintainers provide feedback, please make the necessary changes and update your pull request.

---

## Community

Kmouse is an open-source project maintained by a passionate community. If you have any questions, ideas, or would just like to chat about Kmouse, feel free to:

- Join the discussions on [GitHub Issues](https://github.com/lovenangelo/kmouse/issues)
- Engage with the community on [our social platforms or forums, if applicable]

Thank you for being part of the Kmouse community — your contributions make this project better!

---

We look forward to your contributions!

