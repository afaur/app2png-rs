## Mac App to png

Extract a Mac App's icon to a png file to be used.

### Getting Started

Install [rust](https://www.rust-lang.org/en-US/install.html), then you can build
and run it.

~~~
cargo build
~~~

### Usage

Point it to the path of the `.app` you wish to extract the icon from, and give
it the path you want the icon saved.

~~~
cargo run /Applications/Zazu.app ~/Desktop/Zazu.png
~~~
