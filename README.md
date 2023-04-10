# Rust

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

This repository contains a collection of Rust programs and scripts.

## Table of Contents
- [What is Rust](#what-is-rust)   
- [Installation](#installation)
  - [Installing Rust](#installing-rust)    
    - [Windows](#windows)
    - [Linux](#linux)
    - [macOS](#macOS)  
  - [Installing Repository](#installing-repository)  
- [Usage](#usage)
  - [Default Method](#default-method)
  - [Cargo](#cargo)
- [Contributing](#contributing)
- [License](#license)

## What is Rust

Rust is a modern, multi-paradigm programming language that aims to provide the performance of low-level languages like C and C++ while also providing the safety and memory management of high-level languages like Java and Python. It was created by Mozilla in 2010 and has since gained popularity among developers for its speed, memory safety, and concurrency features.

Rust is a statically-typed language, meaning that the types of variables and expressions are known at compile time, which can catch many errors before the code is run. Rust also includes a powerful ownership and borrowing system, which ensures that memory is allocated and deallocated correctly at compile time, preventing common bugs like null pointer dereferences and use-after-free errors.

Rust has a modern syntax, with features like pattern matching, closures, and type inference, making it easy to write concise and expressive code. It also has a large and growing ecosystem of libraries and tools, including a package manager called Cargo, which makes it easy to manage dependencies and build projects.

Overall, Rust is a powerful language that combines the performance of low-level languages with the safety and ease-of-use of high-level languages. It's used for a wide range of applications, from systems programming and operating systems to web development and game development.

## Installation

### Installing Rust

These are the general steps to install Rust on different platforms. However, the exact steps may vary depending on the specific operating system and version you are using. It's always a good idea to refer to the official documentation for your platform to get more detailed installation instructions.

#### Windows

1. Download the Rust installation executable from the official Rust website.
2. Run the downloaded file and follow the installation instructions.
3. Once the installation is complete, you can use any text editor or IDE (Integrated Development Environment) to write and compile Rust code.

#### Linux

1. Install Rust using the package manager for your Linux distribution. For example, on Ubuntu, you can use the following command:

```
sudo apt-get install rustc
```

2. Once Rust is installed, you can use any text editor or IDE to write and compile Rust code.

#### macOS

1. Install Rust using the Homebrew package manager for macOS. Open a terminal and run the following command:

```
brew install rust
```

2. Once Rust is installed, you can use any text editor or IDE to write and compile Rust code.

Alternatively, you can also download the Rust installation package for macOS from the official Rust website and follow the installation instructions.

### Installing Repository

You can download individual files, copy & paste code, or clone the repository

```
git clone https://github.com/Eggy115/Rust.git
```
      
## Usage

### Default Method

1. Clone the repository. If you have written your own programs, save the program with a `.rs` file extension.
2. Open a terminal or command prompt and navigate to the directory where the Rust program is saved.
3. Run the following command to compile the Rust program:

```
rustc program_name.rs
```

Replace program_name with the name of your Rust program.
This will create an executable file in the same directory as your Rust program.

4. Run the executable file by typing its name into the terminal and pressing Enter.

```
./program_name
```

This will run your Rust program.  

### Cargo

Alternatively, you can use Cargo, Rust's built-in package manager and build tool, to compile and run your Rust program. Here are the steps:

1. Create a new Rust project using Cargo by running the following command in a terminal:

```
cargo new project_name
```

Replace project_name with the name of your project.

2. Open the src directory that was created by Cargo and open the `main.rs` file.
3. Write your Rust program in the `main.rs` file or rename a file from this repository to `main.rs`.
4. Use Cargo to build and run your program by running the following command in a terminal:

```
cargo run
```

This will compile and run your Rust program.

## Contributing

Contributions are always welcome! Follow these steps to contribute:

1. Fork the repository and make your changes. 
2. Submit a pull request with your changes.
3. Create an issue if you find a bug or have a feature request.

Please make sure to adhere to the [code of conduct](CODE_OF_CONDUCT.md) and the [contributing guidelines](CONTRIBUTING.md).

## License

This repository is licensed under the [GPLv3 License](https://www.gnu.org/licenses/gpl-3.0.html). See the [LICENSE](LICENSE) file for more information.
