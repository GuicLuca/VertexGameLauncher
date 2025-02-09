import Download from './download.tsx';
import { useGame } from './gameContext.tsx'
import { convertFileSrc } from '@tauri-apps/api/core';

function gamePage() {
    const { selectedGame } = useGame();
    const isDownloaded = false;
    if (!selectedGame) return <div>Vous n'avez pas de jeux</div>;
    return (
        <div className='infoGame'>
            {/*  background image of the selected game */}
            <img className='background-img' src={convertFileSrc(selectedGame.background_image.local_path)} alt={selectedGame.title} />
            {/* game name */}
            <div className='title'>
                <h1>{selectedGame.title}</h1>
                <h3>{selectedGame.subtitle}</h3>
            </div>
            <div className='infos-jeu'>
                <div className='game-descritpion'>
                    {/* game desc */}
                    <h3>Game Description</h3>
                    <p>{selectedGame.description}</p>
                </div>
                
                {/* TODO: uncoment this section when supporting last update text from the backend */}
                {/* <div> */}
                    {/* infos of the last update */}
                    {/* <h3>Lastest Update</h3> */}
                    {/* <p>Informations sur la dernière mise à jour...</p> */}
                {/* </div> */}
            </div>
            {/* game features */}
            <div className='game-feat'>
                <div>
                    <h3>Tags</h3>
                    <div>
                        {selectedGame.tags.map((os: string, i: number) => (
                            <p key={i}>{os}</p>
                        ))}
                    </div>
                </div>
                {!isDownloaded && (
                    <div>
                        <h3>Système d'exploitation disponible</h3>
                        <div>
                            {selectedGame.platform.map((os: string, i: number) => (
                                <p key={i}>{os}</p>
                            ))}
                        </div>
                    </div>
                )}
            </div>
            <Download />
        </div>
    )
}

export default gamePage