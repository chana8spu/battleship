# Battleship
Relive the classic guessing game, **Battleship**, with a *whole new twist*! This time around, you'll face off against a computer opponent who will be the sole player with a board. Because of that, the way to win *isn't* just to sink all of your opponent's ships! You must also do so within a **certain number of turns**. Each game will be a tense, strategical battle against the computer. And because there's no physical setup, you can jump into a new game immediately!

## How to Run
Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed. Then, download this repository and run the program in a command line interface using ```cargo run``` while in the downloaded directory.

## How to Play

[Watch the video here](https://www.youtube.com/watch?v=YYORyRos64M), or continue on for a text description.

### The Fleet
When the game is run, it starts with the computer player automatically placing down their ships. This is the default fleet:
- Carrier (1x5)
- Battleship (1x4)
- Destroyer (1x3)
- Submarine (1x3)
- Patrol Boat (1x2)
</ul>

Each ship takes up the specified amount of space on a **10x10 grid**. Ships are either placed vertically or horizontally, and cannot intersect. You are unable to see the locations of ships, and must "attack" a space in an attempt to hit a ship. **The goal of the game is sink all five ships before you run out of turns.** A ship sinks when all of its spaces have been attacked.

### The Grid
Each space on the grid is labeled with a letter (A-J) and number (0-9).  To attack a space, type its name (ex. "A4", "C7") then press Enter. Attacking a space uses a turn, and you have 40 turns in total. 
<p>There are three main symbols used in the grid:</p>

- **~** - Water
- **X** - Hit
- **O** - Miss
</ul>

All non-attacked spaces are displayed as water. It's only when you attack a space that you'll see whether a ship was there, via an **X** or an **O**. If you attack the last remaining space of a ship, the ship sinks and you'll be notified:

![Sinking a ship](/images/image1.png)

If you are unable to sink all of the ships within 40 turns, the game is over and any unhit ship spaces will be marked with an **S**:

![Game over](/images/image2.png)

## Data Types in the Program
*(Section numbers referenced in this README and the comments of this program are from "Concepts of Programming Languages", Robert W. Sebesta)*

This program features a wide assortment of data types. Alongside many primitive types (such as ```bool``` and variations of integers), there are a few types of objects worth noting. 

Vectors in Rust are heap-dynamic (6.5.3), meaning they have dynamic storage allocation and are more flexible than other array types. In this program, the positions of ships are modeled with vectors and they dynamically expand as calculations occur. The grid is also modeled with vectors (thus is also dynamic), and although full support for different board sizes is not yet supported, scaling could easily occur without stack overflow.

This program also leverages structs (also known as records (6.7)). Structs provide a simple way to organize the data contained in the program, especially since several different data types can exist in a single struct. And in contrast to other similar types like tuples, fields in structs can be named (6.8).

## Programming the Game
There were a few aspects of the program that were difficult to program. One such aspect was the mechanism to spawn the ships:
- First, they would have to be specific sizes. This is where the vector storing ship sizes came to be. The placement loop uses this vector to spawn the ships one at a time.
- Then, I had some trouble making sure every space of each ship was a valid placement. The syntax to achieve what I wanted is quite complex and closures weren't something I was familiar with before.

Sometimes, knowing how you want to do something but not knowing the syntax is the most frustrating part of coding. And although Rust is more similar to languages like Java and C++ than Scheme and Prolog, there's still some parts that took me a while to understand. But what helped a lot is the error messages. Even though I knew less about the language, errors took far less time to resolve.

## My Experience
As explained in the previous section, I had some difficulty with learning how to code in a new language. But overall, I enjoyed the experience. Game design is something I enjoy a lot, and even though there weren't many original aspects, even just deciding the turn limit was fun. I tested the game many times and decided on a turn limit of 40. Even though it sounds generous, I think a majority of games can't be won within 40 turns. I also got to edit a video, which is one of my hobbies.

And, I learned a lot from doing this project. First, I learned more about the data types that I'm not as familiar with, like records and unions. I also learned preliminary Rust programming and about its unique memory handling.