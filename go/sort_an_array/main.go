package main

import "fmt"

func main(){
    fmt.Printf("%v", sortArray([]int{5,2,3,1}))
}

func sortArray(nums []int) []int {
	if len(nums) <= 1 {
		return nums
	}
    
    fmt.Printf("Nums input: %v\n", nums);

    half_point := int(len(nums)/2)
	
    array_one := nums[0:half_point]
	array_two := nums[half_point:]

    fmt.Printf("Arrays split a: %v b: %v\n", array_one, array_two);
	
    array_one = sortArray(array_one)
	array_two = sortArray(array_two)

    fmt.Printf("Arrays split a post recruse: %v b: %v\n", array_one, array_two);
	return merge(array_one, array_two)
}

func merge(a, b []int) []int {
    fmt.Printf("merge input a: %v, b: %v\n", a, b)
	merged_array := make([]int, len(a)+len(b))
	current_postion := 0
	for {
		if len(a) > 0 && len(b) > 0 {
			if a[0] > b[0] {
				merged_array[current_postion] = b[0]
                fmt.Printf("Added to merged_array: %v\n", merged_array)
				b = b[1:]
				current_postion += 1
                continue
			} else {
				merged_array[current_postion] = a[0]
                fmt.Printf("Added to merged_array: %v\n", merged_array)
				a = a[1:]
				current_postion += 1
                continue
			}
		}

		if len(a) > 0 {
			merged_array[current_postion] = a[0]
            fmt.Printf("Added to merged_array: %v\n", merged_array)
			current_postion += 1
			a = a[1:]
		}

		if len(b) > 0 {
			merged_array[current_postion] = b[0]
            fmt.Printf("Added to merged_array: %v\n", merged_array)
			current_postion += 1
			b = b[1:]
		}

		if len(a) == 0 && len(b) == 0 {
			break
		}
	}
	return merged_array
}
