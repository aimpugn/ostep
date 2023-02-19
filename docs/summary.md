<!-- @format -->

# Summary

- [Summary](#summary)
    - [introduction](#introduction)
        - [가상화(virtualization)](#가상화virtualization)
            - [virtualizing the CPU](#virtualizing-the-cpu)
            - [virtualizing the Memory](#virtualizing-the-memory)
            - [동시성(concurrency)](#동시성concurrency)
            - [저장(persistence)](#저장persistence)

[OSTEP book chapters](https://pages.cs.wisc.edu/~remzi/OSTEP/#book-chapters)

## introduction

### 가상화(virtualization)

OS transform physical into virtual, how does the operating system virtualize resources?

OS를 아래와 같이 부르기도 한다.
1. as virtual machine
2. as resource(CPU, memory, disk) manager

OS API for system calls -> standard library for applications

due to virtualization:
1. run many programs => sharing CPU
2. the programs can **concurrently** access their own instructions and data => sharing memory
3. the progrmas cna access devices => sharing disks and so forth

#### virtualizing the CPU

a single CPU as the seemingly infinite number of CPU.

#### virtualizing the Memory

물리적인 Memory는 bytes 배열.
1. read: 데이터가 저장된 `address`를 명시
2. write(update): 해당 주소에 쓰일 데이터를 명시

프로그램은 데이터를 메모리에 저장하고, load, stores 등의 명령어 통해 메모리에 접근하며, 각 프로그램의 명령(instruction)로 메모리에 있다. 따라서 메모리는 각 명령(instruction)을 가져올 때도 접근하게 된다

여러 프로그램이 각자 메모리 공간을 할당받는다. 해당 공간에 데이터 쓰기를 해도, 이는 각자의 가상 메모리 공간 내에서 이뤄지며, 다른 프로그램의 메모리를 업데이트하지는 않는다. 이는 물리적인 메모리를 다른 프로그램과 공유하기보다는, 각 프로그램이 각자의 private memory(**virtual address space**)를 갖기 때문이다.

#### 동시성(concurrency)

> *Concurrent programming*, where different parts of a program execute independently, and *parallel programming*, where different parts of a program execute at the same time.
>
> 출처: [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

OS는 여러 프로세스를 저글링 하듯 실행시키며, 각 프로세스는 또 여러 쓰레드를 가질 수 있다.

`thread`는,
1. 다른 함수와 같은 메모리 공간에서 실행되는 함수로 생각할 수 있으며,
2. 여러 개의 스레드가 동시에 활성화될 수 있다.

같은 메모리 공간에서 여러 함수가 같이 실행되므로, 만약 공유되는 메모리가 있고 이를 동시에 접근하게 된다면, 의도하지 않은 결과가 나올 수 있다.

```rs
fn worker() {
    for _i in 1..=WORKER_LOOPS {
        // 1. load the value of the counter(shared) from memory into a register
        COUNTER += 1; // 2.one to increment it and, 3. store it back into memory
    }
}
```

위 함수에서 `COUNTER`를 증가시키는 것은 세 가지 명령어(instruction)으로 구성된다
1. 공유되고 있는 `COUNTER`를 메모리에서 레지스터로 불러온다
2. 1을 증가시킨다
3. 증가된 `COUNTER`를 메모리에 저장한다

그런데 이 함수를 여러 쓰레드에서 동시에 실행시킬 떄, 저 세 명령어는 한번에 원자적(**atomically**)으로 실행되지 않는다. 따라서:
1. A 쓰레드에서 1을 증가시킬 때 B 쓰레드에서는 1을 증가시킨 값을 저장하고
2. 그 다음에 A 쓰레드가 증가시킨 값을 저장하면,
3. B가 증가시킨 값은 사라지게 된다

#### 저장(persistence)

CPU나 메모리처럼 애플리케이션마다 가상화된 디스크를 만들지 않는다.
그보다는 오히려 파일의 정보를 **공유**하길 원한다고 가정한다

1. 새로운 파일이 어디에 위치할 것인지 파악하고, 이를 파일 시스템이 유지하는 다양한 구조 속에서 계속 트래킹해야 하며, 이는 더 로우 레벨에 있는 저장 장치에 I/O 요청하는 것이 필요하다
    1. 기존의 구조 읽기
    2. 수정하기
2. device driver를 작성해본 사람이라면 장치로 하여금 뭔가 하게 하는 것은 복잡한 일이며, low level에서의 장치 인터페이스와 정확한 의미에 대한 깊은 지식이 필요하다.
3. OS는 system call을 통해 장치에 접근할 수 있는 표준적이고 간단한 방법을 `standar library`로 제공한다

성능상의 이유로 대부분의 파일시스템은 먼저 이러한 쓰기를 더 큰 그룹으로 일괄 처리하기 위해 잠시 동안 시연시킨다

쓰기 동안의 시스템 충돌을 핸들링하기 위해, 대부분의 파일 시스템은 일종의 복잡한 쓰기 프로토콜인 `journaling` 또는 `copy-on-write`을 포함한다.
만약 쓰기 sequence 동안 실패한다면 나중에 시스템이 합리적인 상태로 복구될 수 있도록 디스크에 대한 쓰기 순서를 신중하게 지정한다.

서로 다른 공통 작업을 효율적으로 만들기 위해, file system은 간단한 목록에서부터 복잡한 b-tree까지 많은 다양한 자료 구조, 액세스 방법을 사용한다.
