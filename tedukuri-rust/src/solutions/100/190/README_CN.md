190\. 字串变换

*    [题目](https://www.acwing.com/problem/content/description/192/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/192/1/)
*    [题解](https://www.acwing.com/problem/content/solution/192/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/192/)

  

已知有两个字串 AA, BB 及一组字串变换的规则（至多 66 个规则）:

A1→B1A1→B1

A2→B2A2→B2

…

规则的含义为：在 AA 中的子串 A1A1 可以变换为 B1B1、A2A2 可以变换为 B2…B2…。

例如：AA＝`abcd` BB＝`xyz`

变换规则为：

`abc` →→ `xu` `ud` →→ `y` `y` →→ `yz`

则此时，AA 可以经过一系列的变换变为 BB，其变换的过程为：

`abcd` →→ `xud` →→ `xy` →→ `xyz`

共进行了三次变换，使得 AA 变换为 BB。

注意，一次变换只能变换一个子串，例如 AA＝`aa` BB＝`bb`

变换规则为：

`a` →→ `b`

此时，不能将两个 `a` 在一步中全部转换为 `b`，而应当分两步完成。

#### 输入格式

输入格式如下：

AA BB  
A1A1 B1B1  
A2A2 B2B2  
… …

第一行是两个给定的字符串 AA 和 BB。

接下来若干行，每行描述一组字串变换的规则。

所有字符串长度的上限为 2020。

#### 输出格式

若在 1010 步（包含 1010 步）以内能将 AA 变换为 BB ，则输出最少的变换步数；否则输出 `NO ANSWER!`。

#### 输入样例：

    abcd xyz
    abc xu
    ud y
    y yz
    

#### 输出样例：

    3
    

难度：中等

时/空限制：1s / 64MB

总通过数：14680

总尝试数：32563

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3741&show_algorithm_tags=0)[NOIP2002提高组](https://www.acwing.com/problem/search/1/?search_content=NOIP2002%E6%8F%90%E9%AB%98%E7%BB%84&source_file_id=3741&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3741&show_algorithm_tags=1)[广度优先搜索](https://www.acwing.com/problem/search/1/?search_content=%E5%B9%BF%E5%BA%A6%E4%BC%98%E5%85%88%E6%90%9C%E7%B4%A2&source_file_id=3741&show_algorithm_tags=1)[BFS](https://www.acwing.com/problem/search/1/?search_content=BFS&source_file_id=3741&show_algorithm_tags=1)[双向BFS](https://www.acwing.com/problem/search/1/?search_content=%E5%8F%8C%E5%90%91BFS&source_file_id=3741&show_algorithm_tags=1)