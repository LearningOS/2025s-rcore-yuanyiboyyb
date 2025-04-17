# chapter 3

## 实现功能

引入一个新的系统调用 ``sys_trace``用来追踪当前任务系统调用的历史信息，通过在syscall中引入全局变量TASK_MANAGER，在每次执行系统调用前遍历任务数组，找到当前正在运行的任务，然后对该任务对应syscall加1，查看历史信息也通过同样的方法

## 简答题

### 1

出现报错
```bash
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
```

ch2b_bad_address.rs 由于除0错误触发异常退出
ch2b_bad_instructions.rs 在用户态非法使用指令
sretch2b_bad_register.rs 在用户态非法使用指令csrr

### 2

#### 2.1

sp指向了内核栈
第一种：执行陷入或终中断后由S态返回U态
第二种：布置用户程序开始运行的环境和恢复切换上下文信息

#### 2.2

```asm
ld t0,32*8(sp) # 内核栈32*8(sp)处存储了原sstatus寄存器的值,将其读取到 t0
ld t1,33*8(sp) # 内核栈32*8(sp)处存储了原sepc寄存器的值,将其读取到 t1
ld t2, 2*8(sp) # 内核栈32*8(sp)处存储了原sscratch寄存器的值,将其读取到t2
csrw sstatus, t0 # 将t0中原sstatus寄存器的值读取到 sstatus
csrw sepc, t1 # 将t1中原sepc寄存器的值读取到sepc
csrw sscratch, t2 # 将t2中原sscratch寄存器的值读取到sscratch
```

#### 2.3

x2是栈指针，已经保存到了sscratch寄存器里，无需恢复
x4是线程指针，还没有线程用不到

#### 2.4

sp指向用户栈, sscratch指向内核栈

#### 2.5

sret指令,CPU会将当前的特权级按照sstatus的SPP字段设置为U,CPU 会跳转到 sepc 寄存器指向的那条指令，然后继续执行。

#### 2.6

sp指向内核栈, sscratch指向用户

#### 2.7

ecall指令

## 荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

一文学懂risc-v汇编操作 https://blog.csdn.net/m0_62730135/article/details/126799687

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。


