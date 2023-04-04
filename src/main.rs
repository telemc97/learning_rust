mod tools;

struct Coordinates{
    latitute: i32,
    longtitude: i32,
    alitute: i32,
}

impl Coordinates{
    fn new()-> Coordinates {
        Coordinates { latitute: 0, longtitude: 0, alitute: 0 }
    }
}

fn main() {
    let vector_capacity:usize = 100;
    let mut vec = Vec::with_capacity(vector_capacity);
    for _ in 0..vector_capacity{
        vec.push(Coordinates::new());
    }
    let mut i:usize = 0;
    let mut num:i32 = 0;

    while i<100 {

        vec[i].latitute = num*tools::get_random_num();
        vec[i].longtitude = num*tools::get_random_num();
        vec[i].alitute = num*tools::get_random_num();

        println!("Latitute {}, Longtitude {}, Altitude {}", vec[i].latitute, vec[i].longtitude, vec[i].alitute);
        num+=1;
        i+=1;
    }
}