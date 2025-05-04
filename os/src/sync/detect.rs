

use alloc::vec::Vec;
use alloc::vec;


///
pub struct DetectInformation{
    #[allow(unused)]
    bool:bool,
    available:Vec<usize>,
    work:Vec<usize>,
    allocate:Vec<Vec<usize>>,
    finish:Vec<bool>,
    need:Vec<Vec<usize>>,
}
impl DetectInformation{
    ///
    pub fn new(bool:bool) -> Self{
        let mut finish = Vec::new();
        finish.push(true);
        Self{
            bool,
            available:Vec::new(),
            work:Vec::new(),
            allocate:(0..10).map(|_| Vec::new()).collect(),
            finish:finish,
            need:(0..10).map(|_| Vec::new()).collect(),
        }
    }
    ///
    pub fn release(&mut self,mutex_id:usize,num:usize,tid:usize){
        //ln!("tid {} release {}",tid,mutex_id);
        self.allocate[tid][mutex_id]-=num;
        self.work[mutex_id]+=num;
    }
    ///
    pub fn get(&mut self,mutex_id:usize,tid:usize,num:usize){
        if self.finish[tid] != true{
            self.finish[tid] = true;
            self.allocate[tid][mutex_id]+=num;
            self.work[mutex_id]-=num;
            self.need[tid][mutex_id]-=num;
            //println!("tid {} finally get {}",tid,mutex_id);
        }
    }
   
    ///
    pub fn get_detect(&mut self,mutex_id:usize,tid:usize,num:usize) -> bool {
        if self.work[mutex_id] < num {
            self.finish[tid]=false;
            self.need[tid][mutex_id]+=num;
            let mut work = self.work.clone();
            let mut finish = self.finish.clone();
            let mut zeros = vec![0;self.finish.len()];
            loop{
                //println!("-------");
                let mut tempflag = false;
                for i in 0..self.finish.len(){
                    if self.finish[i] == true && zeros[i] == 0{
                        tempflag = true;
                        for j in 0..self.available.len(){
                            work[j]+=self.allocate[i][j];
                        }
                        zeros[i] = 1;
                        break;
                    }else if compare(&work,&self.need[i]) && zeros[i] == 0{
                        tempflag = true;
                        for j in 0..self.available.len(){
                            work[j]+=self.allocate[i][j]+self.need[i][j];
                        }
                        finish[i]=true;
                        zeros[i] = 1;
                        break;
                    }
                }
                if !tempflag{
                    break;
                }
            }
            let flag = finish.iter().find(|&&temp|{!temp});
            if flag.is_none(){
                self.finish[tid] = false;
                self.need[tid][mutex_id]+=num;
                //println!("tid {} wait {}",tid,mutex_id);
                true
            }else{
                //println!("tid {} unsafe {}",tid,mutex_id);
                false
            }
        }else{
            self.work[mutex_id]-=num;
            self.allocate[tid][mutex_id]+=num;
            self.finish[tid] = true;
          //  println!("{} safe {} deduct {}",tid,mutex_id,num);
            true
        }
    }
    ///
    pub fn add(&mut self,num:usize,mutex_id:usize){
        if mutex_id >= self.available.len(){
            self.available.push(num);
            self.work.push(num);
         //   println!("{} has {}",mutex_id,num);
            for i in 0..10{
                self.allocate[i].push(0);
                self.need[i].push(0);
            }
        }else{
            self.available[mutex_id]=num;
            self.work[mutex_id] = num;
        }
    }
    ///
    pub fn addtid(&mut self,tid:usize){
        if tid == self.finish.len(){
           // println!("add tid");
            self.finish.push(true);
        }
    }
}
fn compare(vec1:&Vec<usize>,vec2:&Vec<usize>)->bool{
    for i in 0..vec1.len(){
        if vec1[i] < vec2[i]{
            return false;
        }
    }
    true
}