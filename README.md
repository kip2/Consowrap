<h1 align="center"> Consowrap </h1>

consowrap is a command-line utility designed to facilitate the execution of commands with optional arguments and options.

It provides a streamlined interface for running various commands in a consistent manner.

## Features
Execute commands in the form of `consowrap command arg1`, where `command` is the command you wish to run and `arg1` is the argument to the command.

Supports options for commands, allowing you to execute commands like `consowrap command -option arg1`, where `-option` is the option for the command.

## Getting Started
To use consowrap, follow these steps:

1. **Compile for Your Environment**: Ensure that you compile consowrap for your specific environment. This step is crucial to ensure compatibility and performance.
2. **Deploy**: Once compiled, deploy consowrap to your desired location.
3. **Prepare Commands Directory**: Place the executable files of your commands in the `Commands` directory. consowrap requires these executables to be in the `Commands` directory to function properly.

After completing these steps, consowrap will be ready to use with the commands you have set up.
  
<h1 align="center">
Usage
</h1>

## Listing Commands
To list all available commands in consowrap, you can use a specific command designed for this purpose. 

This feature helps users to discover and understand the capabilities of consowrap.

```bash
consowrap -l

# Otherwise
consowrap --list
```

## Executing Commands
To execute a command, use the following syntax:

```bash
consowrap command arg1
```

For commands that require options:

```bash
consowrap command -option arg1
```

<h1 align="center">
Compilation Guide
</h1>
To compile consowrap for your environment, follow these steps:

1. Clone the repository to your local machine.
2. Navigate to the source directory.
3. Run the build script corresponding to your operating system and architecture.
4. Verify the compiled binary in the output directory.


Ensure you have the necessary build tools and dependencies installed before compiling.

<h1 align="center">
Conclusion
</h1>

consowrap is designed to make command execution more efficient and standardized.

By compiling it for your environment and understanding its usage, you can enhance your workflow and productivity.
