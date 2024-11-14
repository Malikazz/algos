package main

import "fmt"

func main() {
	fmt.Printf("%v", sortArray([]int{5, 2, 3, 1}))
}

func sortArray(nums []int) []int {
	return mergeSort(nums)
}

type Node struct {
	value                           int
	child_left, child_right, parent *Node
}

// /Modifys the children of the current node
// /if the children are full it selects the left child
// /modifies its child and returns the child as the current_node
func (node *Node) AddChild(child Node) *Node {
	if node.child_left == nil {
		*node.child_left = child
		return node
	} else if node.child_right == nil {
		*node.child_right = child
		return node
	} else {
		*node.child_left.child_left = child
		return node.child_left
	}
}

func (node *Node) SwapWithParent(child Node) {
	if node.parent == nil {
		return
	}

	if node == node.parent.child_left {
		node, node.parent.child_left = node.parent.child_left, node
	} else {
		node, node.parent.child_right = node.parent.child_right, node
	}
}

func makeHeap(nums []int) *Node {
	var current_node *Node
	var root *Node
	for index := 0; index < len(nums); index++ {
		if current_node == nil {
			current_node := Node{value: nums[index], parent: nil, child_left: nil, child_right: nil}
			root = &current_node
		} else {
			current_node = current_node.AddChild(Node{value: nums[index], parent: current_node, child_left: nil, child_right: nil})
		}
	}
	return root
}

func (node *Node) makeMaxHeap() {
	if node == nil {
		return
	}

	// move to root
	node = node.MoveToRoot()

	if node.child_left.value > node.value {
		node.SwapWithParent(*node.child_left);
	}

}

func (node *Node) MoveToRoot() *Node {
	for {
		if node.parent != nil {
			node = node.parent
		} else {
			break
		}
	}
	return node
}

func heapSort(nums []int) []int {
	current_node := makeHeap(nums)

}

func mergeSort(nums []int) []int {
	if len(nums) <= 1 {
		return nums
	}

	fmt.Printf("Nums input: %v\n", nums)

	half_point := int(len(nums) / 2)

	array_one := nums[0:half_point]
	array_two := nums[half_point:]

	fmt.Printf("Arrays split a: %v b: %v\n", array_one, array_two)

	array_one = sortArray(array_one)
	array_two = sortArray(array_two)

	fmt.Printf("Arrays split a post recruse: %v b: %v\n", array_one, array_two)
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
