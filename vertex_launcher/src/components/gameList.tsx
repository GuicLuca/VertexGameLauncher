import Game from '../models/game.tsx';
import { useGame } from './gameContext.tsx';
import { convertFileSrc } from '@tauri-apps/api/core';

interface GameListProps {
    games: Game[];
}

function gameList({ games }: GameListProps) {

    const { selectedGame, setSelectedGame } = useGame();
    return (
        <div className='gameList'>
            {/* Launcher logo */}
            <img src="/TechNet_Game_Launcher.png" alt="" />
            <span></span>
            {/* Packing of all games*/}
            <div className='games-scroll'>
                <div className='games'>
                    {games.map((game, index) => (
                        <div
                            key={index}
                            className={`game ${selectedGame && selectedGame.title === game.title ? 'active-game' : ''}`}
                            onClick={() => setSelectedGame(game)} // Update the selected game
                        >
                            {/* Logo */}
                            <img src={convertFileSrc(game.navigation_icon.local_path)} alt={`${game.title} logo`} className="game-logo" />
                            <div>
                                {/* Title */}
                                <h3>{game.title}</h3>
                                {/* Version */}
                                <p>Version {game.version}</p>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
            {/* Launcher options */}
            {/* <div className='option btn-strd'>
                Options
            </div> */}
        </div>
    )
}

export default gameList