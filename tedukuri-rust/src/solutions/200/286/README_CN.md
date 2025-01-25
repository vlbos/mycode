286\. 选课

*    [题目](https://www.acwing.com/problem/content/description/288/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/288/1/)
*    [题解](https://www.acwing.com/problem/content/solution/288/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/288/)

  

学校实行学分制。

每门的必修课都有固定的学分，同时还必须获得相应的选修课程学分。

学校开设了 NN 门的选修课程，每个学生可选课程的数量 MM 是给定的。

学生选修了这 MM 门课并考核通过就能获得相应的学分。

在选修课程中，有些课程可以直接选修，有些课程需要一定的基础知识，必须在选了其他的一些课程的基础上才能选修。

例如《Windows程序设计》必须在选修了《Windows操作基础》之后才能选修。

我们称《Windows操作基础》是《Windows程序设计》的先修课。

每门课的直接先修课最多只有一门。

两门课可能存在相同的先修课。

你的任务是为自己确定一个选课方案，使得你能得到的学分最多，并且必须满足先修条件。

假定课程之间不存在时间上的冲突。

#### 输入格式

输入文件的第一行包括两个整数 N、MN、M（中间用一个空格隔开）其中 1≤N≤300,1≤M≤N1≤N≤300,1≤M≤N。

接下来 NN 行每行代表一门课，课号依次为 1，2，…，N1，2，…，N。

每行有两个数（用一个空格隔开），第一个数为这门课先修课的课号（若不存在先修课则该项为 00），第二个数为这门课的学分。

学分是不超过 1010 的正整数。

#### 输出格式

输出一个整数，表示学分总数。

#### 输入样例：

    7 4
    2 2
    0 1
    0 4
    2 1
    7 1
    7 6
    2 2
    

#### 输出样例：

    13
    

难度：中等

时/空限制：1s / 64MB

总通过数：4634

总尝试数：6802

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3837&show_algorithm_tags=0)[《信息学奥赛一本通》算法提高篇](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E4%BF%A1%E6%81%AF%E5%AD%A6%E5%A5%A5%E8%B5%9B%E4%B8%80%E6%9C%AC%E9%80%9A%E3%80%8B%E7%AE%97%E6%B3%95%E6%8F%90%E9%AB%98%E7%AF%87&source_file_id=3837&show_algorithm_tags=0)

算法标签

[动态规划](https://www.acwing.com/problem/search/1/?search_content=%E5%8A%A8%E6%80%81%E8%A7%84%E5%88%92&source_file_id=3837&show_algorithm_tags=1)[背包类树形DP](https://www.acwing.com/problem/search/1/?search_content=%E8%83%8C%E5%8C%85%E7%B1%BB%E6%A0%91%E5%BD%A2DP&source_file_id=3837&show_algorithm_tags=1)