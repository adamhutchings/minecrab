# This is setup for the macOS build process.
# Run this script once per command line session, and thereafter run 'cargo build'
# and 'cargo run' plain to compile and execute the project.

# This is needed for GLFW linkage errors.

# This assumes you are using homebrew for your GLFW installation.

glfw_path=$(brew --prefix glfw)
export RUSTFLAGS="-L $glfw_path/lib -l glfw"
