package task_draw_rect

import "fmt"

func ExampleDrawRectangle() {
	DrawRectangle(3, 3)
	fmt.Println()
	DrawRectangle(3, 6)

	// Output:
	// ###
	// ###
	// ###
	//
	// ######
	// ######
	// ######
}
