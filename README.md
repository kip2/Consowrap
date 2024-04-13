<p align="center">
    <a href="README.md">English</a> | <a href="README-ja.md">日本語</a>
</p>

<h1 align="center"> consowrap </h1>

A simple tool that manages command-line tools and executes commands all in one place.

<h2 align="center">Prerequisites</h2>

- Create a directory to store your command-line tools and place them accordingly.
- This software is not distributed as binary files. Please build it if you wish to use it.

<h2 align="center">Build</h2>

Execute the following in an environment where Rust is installed:

```shell
cargo build --release
```

Place the built executable in your directory. The executable will be generated at the following path:

```shell
./target/release/consowrap
```

### Grant Execution Permissions

Grant execution permissions to the placed file:

```shell
sudo chmod +x consowrap
```

<h2 align="center">Configuration</h2>

### Specify the directory for command-line tools in the `.env` file.

In the `.env` file, specify the directory path where you want to manage your command-line tools. The format is as follows:

```.env
# Format of notation
COMMAND_DRECTORY_PATH="your_commands_directory_path";

# Example
COMMAND_DRECTORY_PATH="./Commands";
```

If a `.env` file does not exist, one will be automatically created. Include the directory path for your command-line tools in the created `.env` file.

<h2 align="center">Usage</h2>

### List Commands

Use the following command to display a list of available commands (files in the directory specified in `.env`):

```shell
consowrap -l

# Or
consowrap --list
```

Note that this will display only the files within the directory specified in `.env`, including non-command-line tools.

### Use Commands

Execute commands in the following format:

```shell
consowrap command arg1 arg2

# If adding options
consowrap command -option arg1 arg2
```

### Help

If you need assistance, call up help:

```shell
consowrap -h

# Or
consowrap --help
```