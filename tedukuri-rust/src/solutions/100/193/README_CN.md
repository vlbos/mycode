193\. 算乘方的牛

*    [题目](https://www.acwing.com/problem/content/description/195/)
*    [讨论](https://www.acwing.com/problem/content/discussion/index/195/1/)
*    [题解](https://www.acwing.com/problem/content/solution/195/1/)
*    [视频讲解](https://www.acwing.com/problem/content/video/195/)

  

约翰的奶牛希望能够非常快速地计算一个数字的整数幂 PP 是多少，这需要你的帮助。

在它们计算得到最终结果的过程中只能保留两个工作变量用于中间结果。

第一个工作变量初始化为 xx，第二个工作变量初始化为 11。

奶牛可以将任意一对工作变量相乘或相除（可以是一个工作变量与自己相乘或相除），并将结果储存在任意一个工作变量中，但是所有结果都只能储存为整数。

举个例子，如果它们想要得到 xx 的 3131 次方，则得到这一结果的一种执行方法如下所示：

                                                工作变量1  工作变量2
    
                                          开始 :   x        1
    
         工作变量1与本身相乘，结果置于工作变量2:   x        x^2
    
         工作变量2与本身相乘，结果置于工作变量2:   x        x^4
    
         工作变量2与本身相乘，结果置于工作变量2:   x        x^8
    
         工作变量2与本身相乘，结果置于工作变量2:   x        x^16
    
         工作变量2与本身相乘，结果置于工作变量2:   x        x^32
    
      工作变量2除以工作变量1，结果置于工作变量2：  x        x^31
    

因此，xx 的 3131 次方经过六个操作就可得到。

现在给出你希望求得的具体次幂数，请你计算至少需要多少个操作才能得到。

#### 输入格式

输入包含一个整数 PP，表示具体次幂数。

#### 输出格式

输出包含一个整数，表示所需最少操作数。

#### 数据范围

1≤P≤200001≤P≤20000

#### 输入样例：

    31
    

#### 输出样例：

    6
    

难度：困难

时/空限制：1s / 30MB

总通过数：654

总尝试数：1610

来源：

[《算法竞赛进阶指南》](https://www.acwing.com/problem/search/1/?search_content=%E3%80%8A%E7%AE%97%E6%B3%95%E7%AB%9E%E8%B5%9B%E8%BF%9B%E9%98%B6%E6%8C%87%E5%8D%97%E3%80%8B&source_file_id=3744&show_algorithm_tags=0)

算法标签

[搜索](https://www.acwing.com/problem/search/1/?search_content=%E6%90%9C%E7%B4%A2&source_file_id=3744&show_algorithm_tags=1)[A\*](https://www.acwing.com/problem/search/1/?search_content=A*&source_file_id=3744&show_algorithm_tags=1)