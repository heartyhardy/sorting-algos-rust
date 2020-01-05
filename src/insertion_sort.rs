
fn insertionsort(nums: &mut [i32]){
    for i in 1..nums.len(){
        let (j, k) = (i-1, nums[i]);
        let mut l = j as i32;
        while l >= 0 && k < nums[l as usize]{
            nums[(l as usize) +1] = nums[l as usize];
            l-=1;
        }
        if l<0 {
            l=0;
            nums[l as usize] = k;
        }else{
            nums[l as usize + 1] = k;
        }
    }
}


pub fn run(){
    let mut nums = [3, 1, 2, 5, 4];
    insertionsort(&mut nums);
    println!("Insertion Sort: {:?}", nums );
}