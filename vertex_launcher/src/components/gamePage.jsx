import React from 'react'
import Download from './download.jsx'
function gamePage() {
    const isDownloaded = false;
    return (
        <div className='gamePage'>
            {/* image background du jeu */}
            <img className='background-img' src="/bg.png" alt="" />
            {/* nom du jeu */}
            <h1>Escape From Tarkov</h1>
            <div className='infos-jeu'>
                <div>
                    {/* description du jeu */}
                    <h3>Game Description</h3>
                    <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Similique possimus corrupti eius et quas, animi debitis totam quibusdam nihil ea eos tempora obcaecati cupiditate hic fuga soluta beatae! Quaerat, asperiores.</p>
                </div>
                <div>
                    {/* information sur la dernière maj */}
                    <h3>Lastest Update</h3>
                    <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolorem debitis sequi fugit magni neque quis, omnis quasi perspiciatis, commodi ducimus soluta architecto amet, ipsa suscipit! Assumenda ipsam quod similique debitis!</p>
                </div>
            </div>
            <div className='game-os'>
                {!isDownloaded &&
                    <div>
                        <h3>Système d'exploitation disponible</h3>
                        <div>
                            <p>Windows</p>
                            <p>MacOS</p>
                            <p>Linux</p>
                        </div>
                    </div>
                }

            </div>
            <div className='start-btn btn-strd'>
                {isDownloaded ? (
                    <div>Start</div>
                ) : (
                    <div>Download</div>
                )}
                {/* version du jeu */}
                <p>V14.25.6235</p>
            </div>
        </div>
    )
}

export default gamePage