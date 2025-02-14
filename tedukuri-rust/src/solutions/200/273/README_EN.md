273\. 分级

+     [题目](https://www.acwing.com/problem/content/description/275/)
+     [讨论](https://www.acwing.com/problem/content/discussion/index/275/1/)
+     [题解](https://www.acwing.com/problem/content/solution/275/1/)
+     [视频讲解](https://www.acwing.com/problem/content/video/275/)

  

给定长度为 $N$ 的序列 $A$，构造一个长度为 $N$ 的序列 $B$，满足：

1.  $B$ 非严格单调，即 $B\_1 \\le B\_2 \\le … \\le B\_N$ 或 $B\_1 \\ge B\_2 \\ge … \\ge B\_N$。
2.  最小化 $S = \\sum\_{i=1}^N|A\_i-B\_i|$。

只需要求出这个最小值 $S$。

#### 输入格式

第一行包含一个整数 $N$。

接下来 $N$ 行，每行包含一个整数 $A\_i$。

#### 输出格式

输出一个整数，表示最小 $S$ 值。

#### 数据范围

$1 \\le N \\le 2000$,  
$0 \\le A\_i \\le 10^6$

#### 输入样例：

```
7
1
3
2
4
5
3
9
```

#### 输出样例：

```
3
```

<table class="table table-striped table-responsive"><tbody><tr><td>难度： <span class="label label-warning round">中等</span></td></tr><tr><td>时/空限制： <span>1s / 64MB</span></td></tr><tr><td>总通过数： <span>4287</span></td></tr><tr><td>总尝试数： <span>7971</span></td></tr><tr><td>来源：</td></tr><tr><td><span class="problem-algorithm-tag">算法标签<span class="caret"></span></span></td></tr></tbody></table>

* * *

代码编辑器设置

* * *

界面风格

对白色界面感到厌倦了吗？可以尝试其他的背景和代码高亮风格。

* * *

编辑类型

更喜欢Vim或者Emacs的输入方式吗？我们也为你提供了这些选项。

* * *

缩进长度

选择代码缩进的长度。默认是4个空格。

* * *

代码补全

写代码疲惫了吗？让我们来帮你一把。唤醒词列表在[这里](https://www.acwing.com/file_system/file/content/whole/index/content/2145234/)。

* * *