/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	//TODO
    let mut cur_pos = 1;
    let max_pos = array.len() - 1;
    while cur_pos <= max_pos {
        let mut pre_pos = cur_pos - 1;
        let mut pos = cur_pos;
        while array[pre_pos] > array[pos] {
            array.swap(pre_pos, pos);
            pos -= 1;      
            if pos == 0 {
                break;
            }    
            pre_pos -= 1;      
        }
        cur_pos += 1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}