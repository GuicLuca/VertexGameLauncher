import React from 'react'
import { useGame } from './gameContext.jsx'

function gamePage() {
    const { selectedGame } = useGame();
    const isDownloaded = false;
    if (!selectedGame) return <div>Chargement du jeu...</div>;
    return (
        <div className='gamePage'>
            {/* image background du jeu */}
            <img className='background-img' src={selectedGame.background_image} alt={selectedGame.title} />
            {/* nom du jeu */}
            <h1>{selectedGame.title}</h1>
            <h3>{selectedGame.subtitle}</h3>
            <div className='infos-jeu'>
                <div>
                    {/* description du jeu */}
                    <h3>Game Description</h3>
                    <p>{selectedGame.description}</p>
                </div>
                <div>
                    {/* information sur la dernière maj */}
                    <h3>Lastest Update</h3>
                    <p>Informations sur la dernière mise à jour...</p>
                </div>
            </div>
            <div className='game-os'>
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