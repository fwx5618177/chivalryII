import { useEffect, useRef } from 'react'
import { createGame } from '@chivalryii/game-engine'

function App() {
  const gameContainerRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    if (!gameContainerRef.current) return

    const config = {
      type: 'web',
      parent: gameContainerRef.current,
      width: 800,
      height: 600,
      backgroundColor: '#000000',
      scale: {
        mode: 'fit',
        autoCenter: true
      }
    }

    const { game, destroy } = createGame(config)

    // 初始化游戏
    game.init()

    return () => {
      destroy()
    }
  }, [])

  return (
    <div className="game-container" ref={gameContainerRef} />
  )
}

export default App;
