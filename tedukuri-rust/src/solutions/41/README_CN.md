41\. 包含min函数的栈










  

设计一个支持push，pop，top等操作并且可以在O(1)时间内检索出最小元素的堆栈。

*   push(x)–将元素x插入栈中
*   pop()–移除栈顶元素
*   top()–得到栈顶元素
*   getMin()–得到栈中最小元素

#### 数据范围

操作命令总数 \[0,100\]。

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

* 剑指Offer 


算法标签

* 单调栈 
