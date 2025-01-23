41\. 包含min函数的栈

*    [题目](https://www.acwing.com/problem/content/description/90/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/90/1/)
*    [题解](https://www.acwing.com/problem/content/solution/90/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/90/)

  

设计一个支持push，pop，top等操作并且可以在O(1)时间内检索出最小元素的堆栈。

*   push(x)–将元素x插入栈中
*   pop()–移除栈顶元素
*   top()–得到栈顶元素
*   getMin()–得到栈中最小元素

#### 数据范围

操作命令总数 \[0,100\]\[0,100\]。

#### 样例

    MinStack minStack = new MinStack();
    minStack.push(-1);
    minStack.push(3);
    minStack.push(-4);
    minStack.getMin();   --> Returns -4.
    minStack.pop();
    minStack.top();      --> Returns 3.
    minStack.getMin();   --> Returns -1.
    

难度：简单

时/空限制：1s / 64MB

总通过数：12308

总尝试数：20038

来源：

[剑指Offer](https://www.acwing.com/problem/search/1/?search_content=%E5%89%91%E6%8C%87Offer&source_file_id=3639&show_algorithm_tags=0)[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3639&show_algorithm_tags=0)[Hulu面试题](https://www.acwing.com/problem/search/1/?search_content=Hulu%E9%9D%A2%E8%AF%95%E9%A2%98&source_file_id=3639&show_algorithm_tags=0)

算法标签

[单调栈](https://www.acwing.com/problem/search/1/?search_content=%E5%8D%95%E8%B0%83%E6%A0%88&source_file_id=3639&show_algorithm_tags=1)