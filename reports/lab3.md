# chapter4

## 实现功能

向前兼容了和``sys_get_time`` ``sys_mmap``和``sys_munmap``系统调用，实现``sys_spawn`` ``sys_setpriority``系统调用，用来创建全新进程和设置进程优先级，同时完成支持stride算法的调度结构

## 简答题

### 1


>问题出在步长的溢出上。由于使用的是 8 位无符号整数，步长的最大值是255。当步长增加时，如果超过了255，就会发生溢出，回到0。因此,p2增加步长变为4，比p1小。

### 2

>如果优先级别都大于二，则步长差值不超过 BigStride / 2，如果步长大的stride值大于步长小的stride值，则这时候步长小的stride值增加，不会产生较大stride值差。如果步长小的stride值大于步长长的，则步长大的stride值加上步长值最多大于步长小的stride值 BigStride / 2

### 3

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let diff = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };

        if diff <= u64::MAX / 2 {
            if self.0 < other.0 {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            if self.0 < other.0 {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与以下各位就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容:

>无

此外，我也参考了以下资料，还在代码中对应的位置以注释形式记录了具体的参考来源及内容:

>无

3.我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4.我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。


