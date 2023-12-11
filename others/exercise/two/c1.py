import numpy as np
import matplotlib.pyplot as plt

# Lotka-Volterra 模型的微分方程
def lotka_volterra(t, y, alpha, beta, delta, gamma):
    x, y = y[0], y[1]
    dx_dt = alpha * x - beta * x * y
    dy_dt = delta * x * y - gamma * y
    return np.array([dx_dt, dy_dt])

# Runge-Kutta 方法
def runge_kutta(func, y0, t, *args):
    n = len(t)
    y = np.zeros((n, len(y0)))
    y[0] = y0
    for i in range(n - 1):
        h = t[i + 1] - t[i]
        k1 = h * func(t[i], y[i], *args)
        k2 = h * func(t[i] + 0.5 * h, y[i] + 0.5 * k1, *args)
        k3 = h * func(t[i] + 0.5 * h, y[i] + 0.5 * k2, *args)
        k4 = h * func(t[i] + h, y[i] + k3, *args)
        y[i + 1] = y[i] + (k1 + 2 * k2 + 2 * k3 + k4) / 6
    return y

# 参数设置
alpha = 0.1  # 猎物出生率
beta = 0.02  # 猎物死亡率
delta = 0.01  # 捕食者增长率
gamma = 0.1  # 捕食者死亡率

# 初始条件
y0 = np.array([40, 9])  # 初始猎物和捕食者数量
t = np.linspace(0, 200, 1000)  # 时间范围和步长

# 使用 Runge-Kutta 方法求解微分方程
result = runge_kutta(lotka_volterra, y0, t, alpha, beta, delta, gamma)

# 绘图
plt.figure(figsize=(10, 6))
plt.plot(t, result[:, 0], label='Prey (x)')
plt.plot(t, result[:, 1], label='Predator (y)')
plt.title('Lotka-Volterra Model')
plt.xlabel('Time')
plt.ylabel('Population')
plt.legend()
plt.grid(True)
plt.show()

