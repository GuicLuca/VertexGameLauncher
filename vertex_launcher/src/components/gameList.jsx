import React, { useState, useEffect } from 'react'
import { useGame } from './gameContext';
function gameList({ games }) {

    const { selectedGame, setSelectedGame } = useGame();
    return (
        <div className='gameList'>
            {/* Launcher logo */}
            <img src="/TechNet_Game_Launcher.png" alt="" />
            <span></span>
            {/* Packing of all games*/}
            <div className='games'>
                {games.map((game, index) => (
                    <div
                        key={index}
                        className={`game ${selectedGame && selectedGame.title === game.title ? 'active-game' : ''}`}
                        onClick={() => setSelectedGame(game)} // Update the selected game
                    >
                        {/* Logo */}
                        <img src={game.navigation_icon} alt={`${game.title} logo`} />
                        <div>
                            {/* Title */}
                            <h3>{game.title}</h3>
                            {/* Version */}
                            <p>Version {game.version}</p>
                        </div>
                    </div>
                ))}
            </div>
            {/* Launcher options */}
            <div className='option btn-strd'>
                Options
            </div>
        </div>
    )
}

export default gameList