fn main() {
    //just in time for christmas
    let song_lines: [&str; 12] = 
    ["A partidge in a pear tree.",
    "Two turtle-doves, and ",
    "Three French hens, ",
    "Four calling birds, ",
    "FIIIIVE GOLDEN RIIIIIIIINGS, ",
    "Six geese a-laying, ",
    "Seven swans a-swimming, ",
    "Eight maids a-milking, ",
    "Nine ladies dancing, ",
    "Ten lords a-leaping, ",
    "Eleven pipers piping, ",
    "Twelve drummers drumming, "];

    let mut i = 0; 

    while i < 12 {
        println!("On the {} day of christmas my true love gave to me.", i + 1);
        println!("{} \n", song_lines[i]);
        i+=1;
    }

}
