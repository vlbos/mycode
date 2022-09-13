/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * type ArrayReader struct {
 * }
 *
 * func (this *ArrayReader) get(index int) int {}
 */

func search(reader ArrayReader, target int) int {
	l, r := 0, 10000
	for l <= r {
		mid := l + (r-l)/2
		if reader.get(mid) == target {
			return mid
		} else if reader.get(mid) > target {
			r = mid - 1
		} else {
			l = mid + 1
		}
	}
	return -1

}