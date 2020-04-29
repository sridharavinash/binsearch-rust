use rand::Rng;

fn main() {
    let mut array = vec![0,24,56,65,78,90,99,120,140,245,567,844,232,199];
    array.sort();
    let key = 1;
    let ret = index_of(&array, key);
    match ret{
        Some(x) => println!("{} is at index {}", key, x),
        None => println!("not found")
    }

    // Test the randomness of 2 6 digit vectors using 
    // binary search
    let v1 = random_vec(100000);
    let v2 = random_vec(100000);
    let mut count = 0;
    for x in v1{
        match index_of(&v2, x){
            Some(_) => {
                count += 1;
            },
            None => ()
        }
    }
    println!("Total Matches: {}, Percent Overlap:{}%", count, (count as f64/v2.len() as f64) * 100.0);
}

fn random_vec(n: usize) -> Vec<u64>{
    let mut v = Vec::with_capacity(n);
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        let num: u64 = rng.gen_range(100000,999999);
        v.push(num);
    }
    v.sort();
    return v;
}

fn index_of(a: &Vec<u64>, k: u64 ) ->  Option<usize>{
    let mut lo = 0;
    let mut hi = a.len()-1;

    while lo <= hi {
        let mid = lo + (hi - lo)/2 ;

        if k > a[mid] {
            lo = mid + 1;
        }else if k < a[mid] {
            if mid == 0 {
                if a[mid] == k{
                    return Some(mid);
                }
                return None;
            }else{
                hi = mid - 1;
            }
        }else{
            return Some(mid);
        }
    }
    return None;
}