use rand::Rng;

pub fn get_random_num()-> i32{
    let number = rand::thread_rng().gen_range(0..101);
    return  number;
}