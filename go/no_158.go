/**
 * The read4 API is already defined for you.
 *
 *     read4 := func(buf4 []byte) int
 *
 * // Below is an example of how the read4 API can be called.
 * file := File("abcdefghijk") // File is "abcdefghijk", initially file pointer (fp) points to 'a'
 * buf4 := make([]byte, 4) // Create buffer with enough space to store characters
 * read4(buf4) // read4 returns 4. Now buf = ['a','b','c','d'], fp points to 'e'
 * read4(buf4) // read4 returns 4. Now buf = ['e','f','g','h'], fp points to 'i'
 * read4(buf4) // read4 returns 3. Now buf = ['i','j','k',...], fp points to end of file
 */

var solution = func(read4 func([]byte) int) func([]byte, int) int {
	// implement read below.
	buf4 := make([]byte, 4)
	l, r := 0, 0 //缓存未输出数据的左下标和右下标的后一位
	return func(buf []byte, n int) int {
		if len(buf) > n {
			buf = buf[:n]
		}
		next := 0
		if l > 0 {
			cnt := copy(buf, buf4[l:r])
			l = (l + cnt) % r
			if l > 0 {
				return cnt
			} else {
				next = cnt
			}
		}
		for next < len(buf) {
			r = read4(buf4)
			if r == 0 {
				return next
			}
			cnt := copy(buf[next:], buf4[:r])
			next += cnt
			l = (l + cnt) % r
		}
		return next
	}
}