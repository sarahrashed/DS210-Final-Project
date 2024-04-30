# DS 210 Final Project Write-Up

## Overview
I used the *Wikispeedia Navigation Paths* dataset provided by SNAP (https://snap.stanford.edu/data/wikispeedia.html) and explored the breadth first search algorithm. Wikispeedia is a game where you are given a starting link and an ending link, and the goal is to traverse Wikipedia pages in order to reach the end link. SNAP provided three files with wikipedia data, of which I used the `wikispeedia_paths-and-graph` file. The file contains `tsv` files of articles, categories, links, game paths (finished and unfinished), and a shortest path distance matrix. I used the files `links.tsv` and `paths_finished.tsv` in order to explore the difference in BFS computed using the algorithm and computed by humans playing the Wikispeedia game. 

## Method
I read in the `links.tsv` and `paths_finished.tsv` files using `reading.rs`. `reading.rs` contains 2 functions `read_link_connections()` and `read_game_connections()`, the first is used for getting and edge list out of `links.tsv` and the second is for getting game data out of `paths_finished.tsv`. I then put everything into graph structure using `graph_structure.rs` which contains 2 functions `directed_adjacency()` and `h_graph_info()`. The first function builds a directed adjacency list out of the edges from `read_link_connections()` and the second builds a vector of the struct `GameData` that stores the seconds of traversal, the full traversal path, the path length, start vertex, end vertex, and difficulty rating given by the player 1-5 (or Null if not given). 

I created `search.rs` to contain the function `wiki_BFS()` which uses the adjacency list, start_link, and end_link to perform BFS on the game paths. I used the basic structure of BFS given in lecture 28, modifying it to work for strings rather than integers. I also altered the function to stop and return the optimal number of steps from the start and end links rather than finding the distance from the start to every other possible connection. 

In main, I took a random sample of 10,000 out of the roughly 50,000 start and end paths contained in `paths_finished.tsv`. I ran BFS on each start/end pair in the sample to get the optimal path length in order to compare it to player traverses. I printed the first 10 traverses from the sample along with its error to visualize some of the information. Beneath the first 10, I printed the average traverse length, the average human traverse length, the MAE, and the most extreme value in the sample. 

## Analysis
Some of the metrics I used were the average traverse length from running BFS on the start and end vertices in the sample, the average traverse length from a player, the MAE of BFS and HBFS (Human Breadth First Search), the averge seconds of bfs, and the average seconds of hbfs. 

I got an average of 2.8 or about 3 steps to get from A to B when using BFS. I got an average of 6.7 or about 7 steps to get from A to B when done by a human. calculated the MAE for the error of bfs to hbfs and got 3.91. I decided to use MAE because error cannot be negative as humans can either match bfs or go over the number of steps never over. MAE is additionally less sensitive to outliers compared to MSE. For example, the path with the largest error from the sample was Archbishop of Canterbury to Love with an error of 63. The MAE of 3.91 means that on average, there is an error of roughly 4 steps between the bfs and hbfs path. Humans are clearly worse at bfs, but by less than one would expect. 

Lastly I computed the average seconds for bfs and hbfs which are 0.0021 seconds and 160.1895 seconds respectively. While the MAE was surprisingly low, the difference in average traversal time is unsurprising. Computers don't have to "think" the same way humans do when traversing a path. Humans need to connect what topics might relate while a computer simply uses structures to compute a for loop. 

Humans are inevitably worse than computers at traversing graphs, but in discussing purely traversal error, they are not far behind. While there are extreme cases, on average humans can find a path going over by about 4 steps. This means that humans have the ability to make topic associations well enough to find a decent path whereas computers compute paths based on literal connections. 

## Code usage
To view the first 10 connections and their error from the sample along with the above described metrics, run `cargo run --release` in main. The paths used to read in `links.tsv` and `paths_finished.tsv` are relative, so there should be no issues reading them. Run the tests at the bottom of main by doing `cargo test`. 

***Note: Resources are stored in a separate file `resources.md`***