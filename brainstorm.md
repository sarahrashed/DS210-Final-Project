# Outline
1. explore data
2. clean data
3. build graph: adjacency and edges list
4. visualize graph
5. build list of attempted connections
    * what are the routes used to get there
    * what are the true best routes to get there
6. compute human error if typically much worse or matches
7. visualize distributions (?)

# Data Exploration
* most useful datasets are `links.tsv` and `paths_finished.tsv`
* might be good to look into how much human error doing bfs compared to given rating of difficulty of path
* might be good to look into `paths_finished.tsv` and compare bfs "difficulty" to the amount of time taken to complete path
    * can analyze `paths_unfished.tsv` to look at how much time on average/nodes traveled through before quitting

# Coding TO DO
* create modules for:
    * building adjacency list
    * building edges list
    * url decoding --> need Rust dependencies