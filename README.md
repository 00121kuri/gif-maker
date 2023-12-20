# GIF Maker
This is a simple command-line tool written in Rust that converts a directory of images into a GIF. The tool allows you to adjust the delay of the first and last frames of the GIF.

## Usage
To use this tool, you need to pass three arguments to the command line:

The directory name where the images are located.  
The delay between frames in the GIF (in hundredths of a second).  
The delay for the first and last frames of the GIF (in hundredths of a second).  
Here is an example of how to use the tool:
```
cargo run <dir_name> 10 100
```
In this example, dir_name is the directory where the images are located, 10 is the delay between frames, and 100 is the delay for the first and last frames.

## Output
The output GIF will be created in the parent directory of the images directory. The name of the GIF will be the same as the name of the images directory.