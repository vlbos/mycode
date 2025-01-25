385\. GF和猫咪的玩具

*    [题目](https://www.acwing.com/problem/content/description/387/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/387/1/)
*    [题解](https://www.acwing.com/problem/content/solution/387/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/387/)

  

GF 同学和猫咪得到了一个特别的玩具，这个玩具由 nn 个金属环（编号为 1∼n1∼n），和 mm 条绳索组成，每条绳索连接两个不同的金属环，并且长度相同。

GF 左手拿起金属环 LL，猫咪右手(或者说:爪)拿起金属环 RR（LL 不等于 RR），然后尽量的向两边拉，他希望选择合适的 LL 和 RR，使得被拉紧的绳索尽量的多。

注：如果像样例那样 1−2−4−3−5−6−11−2−4−3−5−6−1 构成了一个环，我们认为拉 11 和 33 时只能拉紧一边(1−2−4−31−2−4−3 或 3−5−6−13−5−6−1)而不算全部拉紧。

通俗地说，也就是当两个环之间有几个绳索数相等的连接方法时，只算其中一条连接方法拉紧，不算全部拉紧。

#### 输入格式

第一行包含两个正整数 n，mn，m。

接下来的 mm 行包含两个正整数 a，ba，b，表示有一条绳索连接了 aa 和 bb 的绳索。

#### 输出格式

仅包含一个整数，表示最多能拉紧的绳索数。

#### 数据范围

2≤n≤1002≤n≤100

#### 输入样例：

    6 6
    1 2
    1 6
    2 4
    6 5
    4 3
    5 3
    

#### 输出样例：

    3
    

难度：简单

时/空限制：1s / 64MB

总通过数：647

总尝试数：865

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3936&show_algorithm_tags=0)

算法标签

[图论](https://www.acwing.com/problem/search/1/?search_content=%E5%9B%BE%E8%AE%BA&source_file_id=3936&show_algorithm_tags=1)[最短路](https://www.acwing.com/problem/search/1/?search_content=%E6%9C%80%E7%9F%AD%E8%B7%AF&source_file_id=3936&show_algorithm_tags=1)[任意两点间最短路](https://www.acwing.com/problem/search/1/?search_content=%E4%BB%BB%E6%84%8F%E4%B8%A4%E7%82%B9%E9%97%B4%E6%9C%80%E7%9F%AD%E8%B7%AF&source_file_id=3936&show_algorithm_tags=1)