# Lab01

## 运行

```bash
# install rust
curl -L https://static.rust-lang.org/rustup.sh | sudo sh

# install on windows
# https://static.rust-lang.org/dist/rust-nightly-x86_64-pc-windows-gnu.exe

# run
cargo run
```

## 说明

为了寻找合适的 k 值使得绝对误差小于 1.0e-6，先用 Mathematica 来计算几组值：

* x = 1.0, Θ(x) = 1
* x = sqrt(2), Θ(x) = 0.87498299602095037

然后在程序中令 k 最大到 1e6，绝对误差接近 1.0e-6，为了保险起见，将 k 值设为 2e6，最后结果：

* x = 1.00, Θ(x) = 9.999995000000e-1, 绝对误差 5.000000e-7
* x = sqrt(2), Θ(x) = 8.749824960210e-1, 绝对误差 5.000000e-7

