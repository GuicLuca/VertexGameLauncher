import React from 'react'
import { useGame } from './gameContext.jsx'

function gamePage() {
    const { selectedGame } = useGame();
    const isDownloaded = false;
    if (!selectedGame) return <div>Vous n'avez pas de jeux</div>;
    return (
        <div className='gamePage'>
            {/*  background image of the selected game */}
            <img className='background-img' src={selectedGame.background_image} alt={selectedGame.title} />
            {/* game name */}
            <div className='title'>
                <h1>{selectedGame.title}</h1>
                <h3>{selectedGame.subtitle}</h3>
            </div>
            <div className='infos-jeu'>
                <div>
                    {/* game desc */}
                    <h3>Game Description</h3>
                    <p>{selectedGame.description}</p>
                </div>
                <div>
                    {/* infos of the last update*/}
                    <h3>Lastest Update</h3>
                    <p>Informations sur la dernière mise à jour...</p>
                </div>
            </div>
            {/* game features */}
            <div className='game-feat'>
                <div>
                    <h3>Tags</h3>
                    <div>
                        {selectedGame.tags.map((os, i) => (
                            <p key={i}>{os}</p>
                        ))}
                    </div>
                </div>
                {!isDownloaded && (
                    <div>
                        <h3>Système d'exploitation disponible</h3>
                        <div>
                            {selectedGame.platform.map((os, i) => (
                                <p key={i}>{os}</p>
                            ))}
                        </div>
                    </div>
                )}
            </div>
            <div className='start-btn btn-strd'>
                {isDownloaded ? <div>Start</div> : <div>Download</div>}
                <p>V{selectedGame.version}</p>
            </div>
        </div>
    )
}

export default gamePage