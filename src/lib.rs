use std::{collections::HashMap, vec, clone};


struct Node{
    id:u32,
}
impl Node{
    fn new(id:u32)->Self{
        return Self { id:  id}
    }
    
}

struct MatchingGraph{
    anodes:Vec<Node>, //頂点集合左
    bnodes:Vec<Node>,  //頂点集合右

    sides:Vec<[u32;2]>, //辺

    matching_set:Vec<[u32;2]>, //マッチング集合

    incr_roads:Vec<Vec<u32>>,
    incr_road:Vec<u32>,

    
    marked_anode:Vec<u32>,//左側の頂点集合で使用されたもの
    marked_bnode:Vec<u32>//右側の頂点集合で使用されたもの

}



impl MatchingGraph{

    fn new(anodes:Vec<Node>,bnodes:Vec<Node>)->Self{
        let sides:Vec<[u32;2]> = Vec::new();
        let matching_set:Vec<[u32;2]> =Vec::new();
        let incr_roads:Vec<Vec<u32>> = Vec::new();
        let incr_road:Vec<u32> = Vec::new();
        let marked_anode:Vec<u32>=Vec::new();
        let marked_bnode:Vec<u32>=Vec::new();
        Self { 
            anodes: anodes,
            bnodes: bnodes, 
            sides: sides, 
            matching_set: matching_set, 
            incr_roads:incr_roads, 
            incr_road: incr_road, 
            marked_anode: marked_anode, 
            marked_bnode: marked_bnode
        }
    }
    fn add_side(&mut self,anode:u32,bnode:u32){
        self.sides.push([anode,bnode]);
    }
    
    fn get_other_side(&self,node_id:u32,belonging:bool)->Vec<u32>{
        //対岸でペアになりうるノードを返却します
        //python editionと違ってbelonging がboolであることに注意
        return self.sides.iter()
            .filter(|b| b[if belonging{1}else{0}]==node_id)
            .map(|b|b[if belonging{0} else {1}])
            .collect()
    }

    fn init_matching(&mut self){
        //初期マッチング
        //一連のnodeと辺の設定が終わったら
        //マッチング(集合)を初期化します
        for i in self.anodes{
            for j in self.get_other_side(i.id, false){
                todo!();
            }
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0(){
        //test
    }
}
