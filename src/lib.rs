use std::collections::HashMap;


struct Node{
    id:usize,
    //data:HashMap<String,String>
}

///二部グラフに関して有効です
struct MatchingGraph{
    anodes:Vec<Node>, // 0 頂点集合左
    bnodes:Vec<Node>, // 1 頂点集合右

    sides:Vec<(usize,usize)>, //辺
    matching_set:Vec<(usize,usize)>,

    //以下は増加道を発見するために使用
    incr_roads:Vec<Vec<usize>>,
    incr_road:Vec<usize>,

    marked_anode:Vec<usize>,
    marked_bnode:Vec<usize>
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
    fn add_side(&mut self,anode:usize,bnode:usize){
        self.sides.push((anode,bnode));
    }

    /// # get_other_side
    /// return the other side nodes
    fn get_other_side(&self,node_id:usize,belonging:bool)->Vec<usize>{
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
    fn find_unmatching_node(&self,matching:Vec<(usize,usize)>,belonging:bool)->Vec<usize>{
        //python の実装よりも綺麗にする
        if belonging{
            //右側
            let matching_list :Vec<usize>= matching
                .iter()
                .map(|&(_,i)|i).
                collect();
            return self.bnodes.iter()
            .map(|i|i.id)
            .filter(|&i| !matching_list.contains(&i)).collect();
        }else{
            //左側
            let matching_list :Vec<usize>= matching
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
    fn find_matching_node(&self,matching:Vec<(usize,usize)>,belonging:bool)->Vec<usize>{
        todo!()
    }

    fn get_incr_roads2(&mut self,start_node_id:usize)->Vec<Vec<usize>>{
        
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
    fn get_incr_roads_process(&mut self, node_id: usize, belonging: bool, flag: bool) {
        let road = self.incr_road.clone();
        let marked_a_local = self.marked_anode.clone();
        let marked_b_local = self.marked_bnode.clone();
        let next_id = node_id;
    
        if !belonging{
            let mut opposite: Vec<usize> = self.get_other_side(next_id, false)
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
            let mut opposite: Vec<usize> = self.get_other_side(next_id, true)
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
        ///staff_data.0 => name
        ///staff_data.1 => capable
        let staff_data = vec![
            ("1".to_string(),vec![
                                    "B".to_string(),
                                    "D".to_string()
                                ]),
            ("2".to_string(),vec![
                                    "A".to_string(),
                                    "C".to_string(),
                                    "E".to_string()
                                ]),
            ("3".to_string(),vec![
                                    "B".to_string()
                                ]),
            ("4".to_string(),vec![
                                    "D".to_string(),
                                    "E".to_string(),
                                    "F".to_string()
                                ]),
            ("5".to_string(),vec![
                                    "B".to_string(),
                                    "D".to_string()
                                ]),
        ];
        let works_data = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
            "F".to_string(),
        ];
        
        let staff_nodes:Vec<Node> = staff_data
        .iter()
        .enumerate()
        .map(|(i,j)|{
            Node{
                id:i,
                //data:{
                //    let mut map = HashMap::new();
                //    map.insert("name".to_string(), j.0.clone());
                //    map
                //}
            }
        }).collect();
        
        let works_nodes:Vec<Node> = works_data
        .iter()
        .enumerate()
        .map(|(i,j)|
            Node{
                id:i
            }
        ).collect();
        
        let mgraph = MatchingGraph::new(staff_nodes, works_nodes);

    }
}
