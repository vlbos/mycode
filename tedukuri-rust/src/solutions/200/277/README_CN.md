277\. 饼干

*    [题目](https://www.acwing.com/problem/content/description/279/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/279/1/)
*    [题解](https://www.acwing.com/problem/content/solution/279/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/279/)

  

圣诞老人共有 MM 个饼干，准备全部分给 NN 个孩子。

每个孩子有一个贪婪度，第 ii 个孩子的贪婪度为 g\[i\]g\[i\]。

如果有 a\[i\]a\[i\] 个孩子拿到的饼干数比第 ii 个孩子多，那么第 ii 个孩子会产生 g\[i\]×a\[i\]g\[i\]×a\[i\] 的怨气。

给定 N、MN、M 和序列 gg，圣诞老人请你帮他安排一种分配方式，使得每个孩子至少分到一块饼干，并且所有孩子的怨气总和最小。

#### 输入格式

第一行包含两个整数 N,MN,M。

第二行包含 NN 个整数表示 g1∼gNg1∼gN。

#### 输出格式

第一行一个整数表示最小怨气总和。

第二行 NN 个空格隔开的整数表示每个孩子分到的饼干数，若有多种方案，输出任意一种均可。

#### 数据范围

1≤N≤301≤N≤30,  
N≤M≤5000N≤M≤5000,  
1≤gi≤1071≤gi≤107

#### 输入样例：

    3 20
    1 2 3
    

#### 输出样例：

    2
    2 9 9
    

难度：中等

时/空限制：1s / 64MB

总通过数：2605

总尝试数：4804

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3828&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3828&show_algorithm_tags=1)[线性DP](https://www.acwing.com/problem/search/1/?search_content=%E7%BA%BF%E6%80%A7DP&source_file_id=3828&show_algorithm_tags=1)