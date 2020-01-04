
fn selectionsort(nums: &mut [i32]){
    let mut c:usize;
    let mut swp = false;
    for i in 0..nums.len()-1{
        c=i;
        for j in i+1..nums.len(){
            if nums[c] > nums[j]{
                c = j;
                swp=true;
            }
        }
        if swp{
            nums[i] ^= nums[c];
            nums[c] ^= nums[i];
            nums[i] ^= nums[c];
            swp=false;
        }
    }
}

pub fn run(){
    let mut nums: [i32; 5] = [1, 5, 4, 3, 2];

    selectionsort(&mut nums);
    println!("Selection Sort: {:?}", nums );
}