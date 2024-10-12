package task_draw_rect

import "fmt"

func ExampleDrawRectangle_grading() {
	DrawRectangle(5, 4)
	fmt.Println()
	DrawRectangle(1, 8)
	fmt.Println()
	DrawRectangle(3, 1)
	fmt.Println()
	DrawRectangle(0, 0)

	// Output:
	// ####
	// ####
	// ####
	// ####
	// ####
	//
	// ########
	//
	// #
	// #
	// #
	//
}
