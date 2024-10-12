package task_draw_rect

import "fmt"

// Expects two numbers `height` und `width`.
// Draws a rectangle with these dimensions to the console.
// The recangle is completely filled with the symbol '#'.
func DrawRectangle(height, width int) {
	// SOLUTION
	for row := 0; row < height; row++ {
		for col := 0; col < width; col++ {
			fmt.Print("#")
		}
		fmt.Println()
	}
	// SOLUTION_END
}

// HINT
// - Use two nested loops for the rows and columns.
// - The outer loop draws the rows, while the inner loop draws a single entry of a row.
