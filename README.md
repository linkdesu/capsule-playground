在实际的开发中，我们有以下三个独立的仓库

- did-contracts 仓库存放合约源码，需要依赖 did-types ；
- did-types 仓库存放 molecule 编码的类型和一些公共的常量；
- did-toolbox 仓库存放了一些命令行工具，需要依赖 did-types ；

这时通常还会遇到一个问题是，由于 did-types 在 capsule 管理的项目之外，编译合约时不会被映射到容器中，于是导致编译合约找不到 did-types 。
