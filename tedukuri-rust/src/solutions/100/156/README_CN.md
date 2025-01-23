156\. 矩阵

*    [题目](https://www.acwing.com/problem/content/description/158/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/158/1/)
*    [题解](https://www.acwing.com/problem/content/solution/158/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/158/)

  

给定一个 MM 行 NN 列的 0101 矩阵（只包含数字 00 或 11 的矩阵），再执行 QQ 次询问，每次询问给出一个 AA 行 BB 列的 0101 矩阵，求该矩阵是否在原矩阵中出现过。

#### 输入格式

第一行四个整数 M,N,A,BM,N,A,B。

接下来一个 MM 行 NN 列的 0101 矩阵，数字之间没有空格。

接下来一个整数 QQ。

接下来 QQ 个 AA 行 BB 列的 0101 矩阵，数字之间没有空格。

#### 输出格式

对于每个询问，输出 11 表示出现过，00 表示没有出现过。

#### 数据范围

A≤100A≤100，M,N,B≤1000M,N,B≤1000，Q≤1000Q≤1000

#### 输入样例：

    3 3 2 2
    111
    000
    111
    3
    11
    00
    11
    11
    00
    11
    

#### 输出样例：

    1
    0
    1
    

难度：中等

时/空限制：2s / 64MB

总通过数：2193

总尝试数：6052

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3707&show_algorithm_tags=0)[beijing2011](https://www.acwing.com/problem/search/1/?search_content=beijing2011&source_file_id=3707&show_algorithm_tags=0)

算法标签

[字符串hash](https://www.acwing.com/problem/search/1/?search_content=%E5%AD%97%E7%AC%A6%E4%B8%B2hash&source_file_id=3707&show_algorithm_tags=1)[哈希](https://www.acwing.com/problem/search/1/?search_content=%E5%93%88%E5%B8%8C&source_file_id=3707&show_algorithm_tags=1)