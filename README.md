# RlwyNetSim
A project built with Rust to create a Railway network simulator. Ever wanted to build your own railway network and make everything run smooth? Look no more. 

## How to run the Project:

### Windows
> Rust requires the Microsoft C++ build tools for Visual Studio 2013 or later. You can acquire the build tools by installing Microsoft Visual C++ Build Tools 2019 which provides just the Visual C++ build tools:
> https://visualstudio.microsoft.com/visual-cpp-build-tools/
> Check its box for "Desktop development with C++" which will ensure that the Windows 10 SDK is selected. Install the C++ build tools before proceeding.

1. Install Rust by either:
	1. using the rust installer following [these instructions](https://www.rust-lang.org/tools/install)
		1. Upon installation, choose the GNU ABI target (Option3)
	2. OR by running `winget install rustup` in the terminal (either Windows Terminal or cmd) and then restarting the terminal
2. Clone the repository from https://github.com/Ant4ce/RlwyNetSim.git
3. Open the directory of the cloned repo in the terminal
4. switch to the branch `gui-macroquad` with `git checkout gui-macroquad`
5. execute `cargo run`
---
### Ubuntu Linux
1. Make sure you have git, cargo and dependencies installed with `sudo apt-get install git curl clang mold pkg-config libasound2-dev libudev-dev && curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh` and __restart the shell__
	1. Note: when installing rust you can use the default config
2. Navigate to you chosen directory and clone the repository with `git clone https://github.com/Ant4ce/RlwyNetSim.git && cd RlwyNetSim`
3. switch to the branch `gui-macroquad` with `git checkout gui-macroquad`
4. execute `cargo run`

