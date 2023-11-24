use std::collections::HashMap;


struct Node{
    id:u32,
    data:HashMap<String,String>
}

///二部グラフに関して有効です
struct MatchingGraph{
    anodes:Vec<Node>, // 0 頂点集合左
    bnodes:Vec<Node>, // 1 頂点集合右

    sides:Vec<(u32,u32)>, //辺
    matching_set:Vec<(u32,u32)>,

    //以下は増加道を発見するために使用
    incr_roads:Vec<Vec<u32>>,
    incr_road:Vec<u32>,

    marked_anode:Vec<u32>,
    marked_bnode:Vec<u32>
}

impl MatchingGraph{
    fn new(anodes:Vec<Node>,bnodes:Vec<Node>)->Self{
        Self{
            anodes:anodes,
            bnodes:bnodes,

            sides:Vec::new(),

            matching_set:Vec::new(),

            incr_roads:Vec::new(),
            incr_road: Vec::new(),

            marked_anode:Vec::new(),
            marked_bnode:Vec::new()
        }
    }
    /// # add_side
    fn add_side(&mut self,anode:u32,bnode:u32){
        self.sides.push((anode,bnode));
    }

    /// # get_other_side
    /// return the other side nodes
    fn get_other_side(&self,node_id:u32,belonging:bool)->Vec<u32>{
        if belonging {
            self.sides.iter().filter(|&&(a, _)| a == node_id).map(|&(_, b)| b).collect()
        } else {
            self.sides.iter().filter(|&&(_, b)| b == node_id).map(|&(a, _)| a).collect()
        }
    }

    /// # init_matching
    /// After all settings are complete, initialize the matching set
    fn init_matching(&mut self){
        self.matching_set = Vec::new();//ここの初期化は絶対
        for i in &self.anodes{
            for j in self.get_other_side(i.id,false){
                if self.matching_set.iter().all(|&(_,a)|a!=j){
                    self.matching_set.push((i.id,j));
                    break;
                }
            }
        }
    }

    /// # find_unmatching_node
    /// arg `matching` is matched node pair list
    /// This method return unmatch nodes
    fn find_unmatching_node(&self,matching:Vec<(u32,u32)>,belonging:bool)->Vec<u32>{
        //python の実装よりも綺麗にする
        if belonging{
            //右側
            let matching_list :Vec<u32>= matching
                .iter()
                .map(|&(_,i)|i).
                collect();
            return self.bnodes.iter()
            .map(|i|i.id)
            .filter(|&i| !matching_list.contains(&i)).collect();
        }else{
            //左側
            let matching_list :Vec<u32>= matching
                .iter()
                .map(|&(_,i)|i).
                collect();
            return self.bnodes.iter()
            .map(|i|i.id)
            .filter(|&i| matching_list.contains(&i)).collect();
        }
    }

    /// # find_matching_node
    /// あまり使わないため保留
    fn find_matching_node(&self,matching:Vec<(u32,u32)>,belonging:bool)->Vec<u32>{
        todo!()
    }

    fn get_incr_roads2(&mut self,start_node_id:u32)->Vec<Vec<u32>>{
        
        //それぞれの変数の初期化
        self.incr_roads=Vec::new();
        self.incr_road =Vec::new();

        self.marked_anode=Vec::new();
        self.marked_bnode=Vec::new();

        self.marked_anode.push(start_node_id);
        self.get_incr_roads_process(
            start_node_id,
            false,
            false
        );

        self.incr_roads.clone()
    }

    /// この関数は普通プログラマーが触る必要のない部分であり、且つマッチングアルゴリズムのコアでもある
    /// node引数はマッチしていないものでanodesに属するものを選ぶ必要がある
    /// 
    /// 返り値は増ないが、この関数の実行直後のself.incr_roadsは増加道を複数含んだVecである
    fn get_incr_roads_process(&mut self, node_id: u32, belonging: bool, flag: bool) {
        let road = self.incr_road.clone();
        let marked_a_local = self.marked_anode.clone();
        let marked_b_local = self.marked_bnode.clone();
        let next_id = node_id;
    
        if !belonging{
            let mut opposite: Vec<u32> = self.get_other_side(next_id, false)
                .into_iter()
                .filter(|&i| !road.iter().step_by(2).any(|&x| x == i))
                .filter(|&j| !self.matching_set.contains(&(next_id, j)))
                .filter(|&k| !self.marked_bnode.contains(&k))
                .collect();
    
            if !opposite.is_empty() {
                self.marked_bnode.append(&mut opposite);
                for &i in &opposite {
                    self.incr_road.push(i);
                    self.get_incr_roads_process(i, true, true);
                    self.incr_road = road.clone();
                    self.marked_anode = marked_a_local.clone();
                }
            } else if flag {
                self.incr_roads.push(self.incr_road.clone());
            }
        } else {
            let mut opposite: Vec<u32> = self.get_other_side(next_id, true)
                .into_iter()
                .filter(|&i| !road.iter().skip(1).step_by(2).any(|&x| x == i))
                .filter(|&j| self.matching_set.contains(&(j, next_id)))
                .filter(|&k| !self.marked_anode.contains(&k))
                .collect();
    
            if !opposite.is_empty() {
                self.marked_anode.append(&mut opposite);
                for &i in &opposite {
                    self.incr_road.push(i);
                    self.get_incr_roads_process(i, false, true);
                    self.incr_road = road.clone();
                    self.marked_bnode = marked_b_local.clone();
                }
            } else {
                self.incr_roads.push(self.incr_road.clone());
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {

    }
}
