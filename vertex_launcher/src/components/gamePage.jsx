import React from 'react'
import Download from './download.jsx'
function gamePage() {
    return (
        <div className='gamePage'>
            {/* iamge background du jeu */}
            <img className='background-img' src="/bg.png" alt="" />
            {/* nom du jeu */}
            <h1>Escape From Tarkov</h1>
            <div className='infos-jeu'>
                <div>
                    {/* description du jeu */}
                    <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Similique possimus corrupti eius et quas, animi debitis totam quibusdam nihil ea eos tempora obcaecati cupiditate hic fuga soluta beatae! Quaerat, asperiores.</p>
                </div>
                <div>
                    {/* information sur la dernière maj */}
                    <p>Lorem ipsum dolor sit amet consectetur adipisicing elit. Dolorem debitis sequi fugit magni neque quis, omnis quasi perspiciatis, commodi ducimus soluta architecto amet, ipsa suscipit! Assumenda ipsam quod similique debitis!</p>
                </div>
            </div>
            <div className='start-btn'>
                <Download />
                {/* version du jeu */}
                <p>V 14.25.6235</p>
            </div>
        </div>
    )
}

export default gamePage