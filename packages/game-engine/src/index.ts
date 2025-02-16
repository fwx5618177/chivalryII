import { Game } from './core/Game'
import type { GameConfig, GameInstance } from './types'

export function createGame(config: GameConfig): GameInstance {
  const game = new Game(config)
  
  return {
    game,
    destroy: () => game.destroy()
  }
}

export type { GameConfig, GameInstance }
export { Game } 