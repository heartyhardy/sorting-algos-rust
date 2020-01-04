
fn bubblesort(nums: &mut [i32]){
    let (mut swp, mut l, mut i) = (false, nums.len()-1, 0);
    while l > 0{
        if nums[i] > nums[i+1]{
            nums[i] ^= nums[i+1];
            nums[i+1] ^= nums[i];
            nums[i] ^= nums[i+1];
            swp=true;
        }        
        i+=1;
        if i == l && !swp{
            break;
        }else if i == l && swp{
            i = 0;
            l-=1;
            swp=false;            
        }
    }
}

pub fn run(){
    let mut nums: [i32;5] = [4, 5, 2, 3, 1];

    bubblesort(&mut nums);
    println!("BubbleSort: {:?}", nums);
}