mod huffman;

fn main() {
    println!("Hello, world!");
}
/*
HOW TO:
get total amt of each char in a given string
combine two least seen char's into a string "c + b -> cb | value
make a dict for each char and occurences
now do it again (combine two least common) e + cb -> ecb | value
do this until you have one string
this should be a binary tree of the single characters

each time there is a left, type 0
each time right, type 1
trace a path using 1's and 0's to each char (c = 1110)
concat back into string of 0's and 1's
 */