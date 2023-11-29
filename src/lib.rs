use std::collections::HashMap;
use wasm_bindgen::prelude::*;

///二部グラフに関して有効です
#[wasm_bindgen]
pub struct MatchingGraph{
    anodes:Vec<usize>, // 0 頂点集合左
    bnodes:Vec<usize>, // 1 頂点集合右

    sides:Vec<(usize,usize)>, //辺
    matching_set:Vec<(usize,usize)>,

    //以下は増加道を発見するために使用
    incr_roads:Vec<Vec<usize>>,
    incr_road:Vec<usize>,

    marked_anode:Vec<usize>,
    marked_bnode:Vec<usize>
}

#[wasm_bindgen]
impl MatchingGraph{
    #[wasm_bindgen(constructor)]
    pub fn new(
        anodes:Vec<usize>,
        bnodes:Vec<usize>
    )->Self
    {
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
    #[wasm_bindgen(js_name = addSide)]
    pub fn add_side(&mut self,anode:usize,bnode:usize){
        self.sides.push((anode,bnode));
    }

    /// # get_other_side
    /// return the other side nodes
    fn get_other_side(
        &self,
        node_id:usize,
        belonging:bool
    )->Vec<usize>
    {
        if !belonging {
            //左側から右側に進めるノードを探す
            self.sides.iter()
            .filter(|&&(a, _)| a == node_id)
            .map(|&(_, b)| b)
            .collect()
        } else {
            // 右側から進める左ノードを探す
            self.sides.iter()
            .filter(|&&(_, b)| b == node_id)
            .map(|&(a, _)| a)
            .collect()
        }
    }

    /// # init_matching
    /// After all settings are complete, initialize the matching set
    fn init_matching(&mut self){
        self.matching_set = Vec::new();//ここの初期化は絶対
        for &i in &self.anodes{
            for j in self.get_other_side(i,false){
                if self.matching_set.iter().all(|&(_,a)|a!=j){
                    self.matching_set.push((i,j));
                    break;
                }
            }
        }
    }

    /// # find_unmatching_node
    /// arg `matching` is matched node pair list
    /// This method return unmatch nodes
    fn find_unmatching_node(
        &self,
        matching:&Vec<(usize,usize)>,
        belonging:bool
    )->Vec<usize>
    {
        //python の実装よりも綺麗にする
        if belonging{
            //右側
            let matching_list :Vec<usize>= matching
                .iter()
                .map(|&(_,i)|i).
                collect();
            self.bnodes.iter()
            .map(|&i|i)
            .filter(|&i| !matching_list.contains(&i)).collect()
        }else{
            //左側
            let matching_list :Vec<usize>= matching
                .iter()
                .map(|&(i,_)|i).
                collect();
            self.anodes.iter()
            .map(|&i|i)
            .filter(|&i| !matching_list.contains(&i)).collect()
        }
    }

    /// # find_matching_node
    /// あまり使わないため保留
    fn find_matching_node(
        &self,
        matching:Vec<(usize,usize)>,
        belonging:bool
    )->Vec<usize>
    {
        todo!()
    }

    /// # get_incr_roads
    /// 左側にある、まだマッチしていないnodeのidを引数にとります
    /// 増加道かまたは変更可能なノード先を返却します
    fn get_incr_roads(
        &mut self,
        start_node_id:usize
    )->Vec<Vec<usize>>
    {

        //変数の初期化
        self.incr_roads=Vec::new();
        self.incr_road=Vec::new();

        self.marked_anode=Vec::new();
        self.marked_bnode=Vec::new();

        self.marked_anode.push(start_node_id);
        self.get_incr_roads_process(
            start_node_id,
            false,
            true
        );
        self.incr_roads.clone()
    }

    /// # get_incr_road2
    /// 左側にある、まだマッチしていないnodeのidを引数にとります
    /// 増加道かまたは変更可能なノード先を返却します
    fn get_incr_roads2(
        &mut self,
        start_node_id:usize
    )->Vec<Vec<usize>>
    {
        
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
    fn get_incr_roads_process(
        &mut self,
        node_id: usize,
        belonging: bool,
        flag: bool
    ) {
        let road = self.incr_road.clone();
        let marked_a_local = self.marked_anode.clone();
        let marked_b_local = self.marked_bnode.clone();
        let next_id = node_id;

        if !belonging{
            let mut opposite: Vec<usize> = self.get_other_side(next_id, false);
            opposite=opposite.into_iter()
                .filter(|&i| !road.iter().step_by(2).any(|&x| x == i))
                .filter(|&j| !self.matching_set.contains(&(next_id, j)))
                .filter(|&k| !self.marked_bnode.contains(&k))
                .collect();

            if !opposite.is_empty() {
                for &i in &opposite {
                    self.marked_bnode.extend(&opposite);//Vecの連結
                    self.incr_road.push(i);
                    self.get_incr_roads_process(i, true, true);
                    self.incr_road = road.clone();
                    self.marked_anode = marked_a_local.clone();
                }
            } else if flag {
                //self.incr_roads.push(self.incr_road.clone());
            }else{
                //println!("{:?}",self.incr_road);
            }
        } else {
            let mut opposite: Vec<usize> = self.get_other_side(next_id, true);
            opposite=opposite.into_iter()
                .filter(|&i| !road.iter().skip(1).step_by(2).any(|&x| x == i))
                .filter(|&j| self.matching_set.contains(&(j, next_id)))
                .filter(|&k| !self.marked_anode.contains(&k))
                .collect();

            if !opposite.is_empty() {
                for &i in &opposite {
                    self.marked_anode.extend( &opposite);//Vecの連結
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

    /// # incr_side_iter
    /// スタートノードと増加道Vecを入力すると辺の情報になったVecが返却されます
    fn incr_side_iter(
        &mut self,
        start_node_id:usize,
        incr_list:&Vec<usize>
    )->Vec<(usize,usize)>
    {
        let mut incr_road_map = vec![start_node_id];
        incr_road_map.extend(incr_list);
        let rlist :Vec<(usize,usize)>= incr_road_map[0..incr_road_map.len()-1]
        .iter()
        .enumerate()
        .map(|(i,&j)|
        if i%2==0 {
            (j,incr_road_map[i+1])
        }else{
            (incr_road_map[i+1],j)
        }).collect();
        rlist
    }

    fn new_matching_set_creator(
        &self,
        matching             :&Vec<(usize,usize)>,
        remove_matching_set  :Vec<(usize,usize)>,
        mut add_matching_set :Vec<(usize,usize)>
    )                       ->Vec<(usize,usize)>{
        let mut rlist:Vec<(usize,usize)> = matching
            .iter()
            .filter(|&&(a,b)|!remove_matching_set.contains(&(a,b)))
            .cloned()
            .collect();
        rlist.append(&mut add_matching_set);
        rlist
    }

    #[wasm_bindgen(js_name = maxMatching)]
    pub fn max_matching(&mut self)->Vec<(usize,usize)>{
        self.init_matching();
        loop {
            let unmatching_list = self.find_unmatching_node(&self.matching_set, false);
            if unmatching_list.is_empty(){
                return JsValue::from_str(&serde_json::to_string(&self.matching_set).unwrap());//仮止め
            }
                
            let mut incriment:Vec<Vec<usize>> = self.get_incr_roads(unmatching_list[0]);
            incriment=incriment
                .iter()
                .filter(|&i|i.len()>2)
                .cloned()
                .collect();



            if incriment.is_empty(){
                return JsValue::from_str(&serde_json::to_string(&self.matching_set).unwrap());//仮止め
            }else{
                let incr_road = self.incr_side_iter(
                    unmatching_list[0],
                    &incriment[0]);
                

                let remove_matching_set = incr_road
                .iter()
                .skip(1)
                .step_by(2)
                .map(|&i|i)
                .collect();
                
                let add_matching_set= incr_road
                .iter()
                .step_by(2)
                .map(|&i|i)
                .collect();
                
                self.matching_set = self.new_matching_set_creator(
                    &self.matching_set,
                    remove_matching_set,
                    add_matching_set
                );
            }

        }
    }

    ///maxかどうか関係なくマッチングを返却します
    fn max_matching2(&mut self)->Vec<Vec<(usize,usize)>>{
        let mut rlist = Vec::new();
        self.init_matching();
        let unmatching_list = self.find_unmatching_node(&self.matching_set, false);
        
        for i in unmatching_list{
            let increment = self.get_incr_roads(i);
            for inc in increment{
                let incr_road = self.incr_side_iter(i, &inc);
                let remove_matching_set = incr_road
                .iter()
                .skip(1)
                .step_by(2)
                .map(|&i|i)
                .collect();
                let add_matching_set = incr_road
                .iter()
                .step_by(2)
                .map(|&i|i)
                .collect();
                let changed_matching = self.new_matching_set_creator(
                    &self.matching_set,
                    remove_matching_set, 
                    add_matching_set
                );
                rlist.push(changed_matching);
            }
        }
        return rlist;
    }
}


//testをスムーズにしたいのでwasm用関数に分割しています


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        // staff_data.0 => name
        // staff_data.1 => capable
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
        
        let staff_nodes:Vec<usize> = staff_data
        .iter()
        .enumerate()
        .map(|(i,_)|{
            i
        }).collect();
        
        let works_nodes:Vec<usize> = works_data
        .iter()
        .enumerate()
        .map(|(i,_)|
            i
        ).collect();
        
        let mut mgraph = MatchingGraph::new(staff_nodes, works_nodes);

        for (i, j) in staff_data.iter().enumerate(){
            for k in j.1.clone(){
                let indexof = works_data.iter().position(|l| l == &k).unwrap();
                mgraph.add_side(i, indexof);
            }
        }

        println!(
            "最大マッチング {:?}",
            mgraph.max_matching()
        );

        println!(
            "{:?}",
            mgraph.max_matching2()
        )

    }
    #[test]
    fn test1(){
        use std::fs::File;
        use std::io::prelude::*;
        use serde::{Serialize, Deserialize};
    
        #[derive(Serialize, Deserialize, Debug)]
        struct member{
            name:String,
            capable:Vec<String>
        }

        let filename0 ="../data/02/staff.json"; 
        let filename1 = "../data/02/works.json";
        println!("In file {}", filename0);
        println!("In file {}", filename1);
    
        // ファイルが見つかりませんでした
        let mut f0 = File::open(filename0).expect("file not found");
        let mut f1 = File::open(filename1).expect("file not found");
        let mut contents0 = String::new();
        let mut contents1 = String::new();
        f0.read_to_string(&mut contents0)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");
        f1.read_to_string(&mut contents1)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");
        let deserialized0 :Vec<member>= serde_json::from_str(&contents0).unwrap();
        let deserialized1 :Vec<String>= serde_json::from_str(&contents1).unwrap();
        // テキストは\n{}です
        for data in &deserialized0 {

            println!("{:?}", data);
        }
        for work in &deserialized1{
            println!("{:?}", work);
        }

        let staff_nodes:Vec<usize> = deserialized0
        .iter()
        .enumerate()
        .map(|(i,_)|{
            i
        }).collect();
        
        let works_nodes:Vec<usize> = deserialized1
        .iter()
        .enumerate()
        .map(|(i,_)|
            i
        ).collect();
        
        let mut mgraph = MatchingGraph::new(staff_nodes, works_nodes);

        let mut index = 0;
        for i in &deserialized0{
            for k in &i.capable{
                let indexof = deserialized1.iter().position(|l| l == k).unwrap();
                mgraph.add_side(index, indexof);
            }
            index+=1;
        }

        println!("{:?}",mgraph.max_matching());
    }
}
