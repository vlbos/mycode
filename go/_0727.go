func minWindow(s1 string, s2 string) string {
	if s1 == "" || s2 == "" || len(s1) < len(s2) {
		return ""
	}
	rer, rel := math.MaxInt32, 0 //返回值的坐标
	i, j := 0, 0                 // 分别作为s1，s2的坐标
	for i < len(s1) {
		if s1[i] == s2[j] {
			j++
		}
		if j == len(s2) {
			right := i
			j--
			for j >= 0 {
				if s2[j] == s1[i] {
					j--
				}
				i--
			}
			i++
			if right-i < rer-rel {
				rer = right
				rel = i
			}
			j = 0
		}
		i++
	}
	if rer == math.MaxInt32 {
		return ""
	}
	return s1[rel : rer+1]

}