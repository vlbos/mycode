/**
 * // This is the robot's control interface.
 * // You should not implement it, or speculate about its implementation
 * type Robot struct {
 * }
 *
 * // Returns true if the cell in front is open and robot moves into the cell.
 * // Returns false if the cell in front is blocked and robot stays in the current cell.
 * func (robot *Robot) Move() bool {}
 *
 * // Robot will stay in the same cell after calling TurnLeft/TurnRight.
 * // Each turn will be 90 degrees.
 * func (robot *Robot) TurnLeft() {}
 * func (robot *Robot) TurnRight() {}
 *
 * // Clean the current cell.
 * func (robot *Robot) Clean() {}
 */

func cleanRoom(robot *Robot) {
	var directions = [][2]int{[2]int{-1, 0}, [2]int{0, 1}, [2]int{1, 0}, [2]int{0, -1}}
	var visited = make(map[int]map[int]bool)
	var back = func() {
		robot.TurnRight()
		robot.TurnRight()
		robot.Move()
		robot.TurnRight()
		robot.TurnRight()
	}
	var backtrack func(r, c, d int)
	backtrack = func(r, c, d int) {
		if _, ok := visited[r]; !ok {
			visited[r] = make(map[int]bool)
		}
		visited[r][c] = true
		robot.Clean()
		for i := 0; i < 4; i++ {
			newD := (d + i) % 4
			newR := r + directions[newD][0]
			newC := c + directions[newD][1]
			if v, ok := visited[newR]; (!ok || !v[newC]) && robot.Move() {
				backtrack(newR, newC, newD)
				back()
			}
			robot.TurnRight()
		}
	}
	backtrack(0, 0, 0)
}