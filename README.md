# pngme
Encode messages into a png file and then decode them

Resources: [Project Book](https://picklenerd.github.io/pngme_book/introduction.html) and [The PNG Spec](http://www.libpng.org/pub/png/spec/1.2/PNG-Introduction.html)

For learning purposes.

# Installing
Make sure rust is installed on your computer

Clone the repository and enter the direcotry

Run

`$ cargo build --release`

and then navigate to 

`pngme/target/release/`

And then move the executable to your desired location 

# Running
There are 4 subcommands 
1. encode \<FILE\> \<CHUNKTYPE\> \<MESSAGE\> [OUTPUT],
2. decode \<FILE\> \<CHUNKTYPE\>
3. remove \<FILE\> \<CHUNKTYPE\>
4. print \<FILE\>
  
Quick Note: A chunktype is a string with 4 characters (alphebatical only) where the 3rd letter MUST be capitalized
  
Have fun
