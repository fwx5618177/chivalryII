export type PlatformType = 'web' | 'electron' | 'mobile'

export interface GameConfig {
  type: PlatformType
  parent: HTMLElement
  width: number
  height: number
  backgroundColor?: string
  scale?: {
    mode: 'fit' | 'stretch'
    autoCenter: boolean
  }
}

export interface Game {
  init: () => void
  update: () => void
  render: () => void
}

export interface GameInstance {
  game: Game
  destroy: () => void
} 