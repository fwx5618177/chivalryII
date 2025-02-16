import { Game as IGame, GameConfig } from '../types'

export class Game implements IGame {
  private canvas: HTMLCanvasElement
  private ctx: CanvasRenderingContext2D
  private config: GameConfig
  private animationFrameId: number | null = null

  constructor(config: GameConfig) {
    this.config = config
    this.canvas = document.createElement('canvas')
    this.canvas.width = config.width
    this.canvas.height = config.height
    
    const context = this.canvas.getContext('2d')
    if (!context) {
      throw new Error('Failed to get 2D context')
    }
    this.ctx = context
  }

  init() {
    // 设置背景色
    if (this.config.backgroundColor) {
      this.ctx.fillStyle = this.config.backgroundColor
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height)
    }

    // 添加到父容器
    this.config.parent.appendChild(this.canvas)

    // 开始游戏循环
    this.gameLoop()
  }

  update() {
    // 这里添加游戏逻辑更新
  }

  render() {
    // 清除画布
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height)
    
    // 重新填充背景
    if (this.config.backgroundColor) {
      this.ctx.fillStyle = this.config.backgroundColor
      this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height)
    }

    // 在这里添加渲染逻辑
    // 示例：绘制一个简单的矩形
    this.ctx.fillStyle = '#ff0000'
    this.ctx.fillRect(
      this.canvas.width / 2 - 25,
      this.canvas.height / 2 - 25,
      50,
      50
    )
  }

  private gameLoop = () => {
    this.update()
    this.render()
    this.animationFrameId = requestAnimationFrame(this.gameLoop)
  }

  destroy() {
    if (this.animationFrameId !== null) {
      cancelAnimationFrame(this.animationFrameId)
    }
    this.canvas.remove()
  }
} 