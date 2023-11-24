use std::collections::HashMap;

struct Node{
    id:u32,
    data:HashMap<String,String>
}

struct MatchingGraph{
    anode:Vec<Node>, // 0 頂点集合左
    bnode:Vec<Node>, // 1 頂点集合右

    side:Vec<(u32,u32)>, //辺
    matching_set:Vec<(u32,u32)>,

    //以下は増加道を発見するために使用
    incr_roads:Vec<Vec<u32>>,
    incr_road:Vec<u32>,

    marked_anode:Vec<u32>,
    marked_bnode:Vec<u32>
}
