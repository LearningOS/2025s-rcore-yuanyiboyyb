use alloc::vec::Vec;


///
pub struct DetectInformation{
    available:Vec<isize>,
}
impl DetectInformation{
    ///
    pub fn new() -> Self{
        Self{
            available:Vec::new(),
        }
    }
    fn get(&mut self,mutex_id:usize,num:usize){
        self.available[mutex_id]-=num as isize;
    }
    ///
    pub fn release(&mut self,mutex_id:usize,num:usize){
        self.available[mutex_id]+=num as isize;
    }
    fn detect(&self)->bool{
        for i in &self.available{
            if *i < 0{
                return false;
            }
        }
        true
    }
    ///
    pub fn get_detect(&mut self,mutex_id:usize,num:usize)->bool{
        self.get(mutex_id,num);
        self.detect()
    }
    ///
    pub fn add(&mut self,num:usize,mutex_id:usize){
        if mutex_id >= self.available.len(){
            self.available.push(num as isize);
        }else{
            self.available[mutex_id]=num as isize;
        }
    }
}