import React from 'react'

function gameList() {
  return (
    <div className='gameList'>
        {/* logo du launcher */}
        <img src="/TechNet_Game_Launcher.png" alt="" />
        <span></span>
        {/* regroupement de tous les jeux */}
        <div>
            {/* Jeu */}
            <div className='jeu'>
                {/* logo */}
                <img src="/TechNet_Game_Launcher.png" alt="" />
                <div>
                    {/* titre */}
                    <h3>Escape from Tarkov</h3>
                    {/* version */}
                    <p>V 1.14.5423</p> 
                </div>
            </div>
            {/* Jeu */}
            <div className='jeu'>
                {/* logo */}
                <img src="/TechNet_Game_Launcher.png" alt="" />
                <div>
                    {/* titre */}
                    <h3>Escape from Tarkov</h3>
                    {/* version */}
                    <p>V 1.14.5423</p> 
                </div>
            </div>
            {/* Jeu */}
            <div className='jeu'>
                {/* logo */}
                <img src="/TechNet_Game_Launcher.png" alt="" />
                <div>
                    {/* titre */}
                    <h3>Escape from Tarkov</h3>
                    {/* version */}
                    <p>V 1.14.5423</p> 
                </div>
            </div>
        </div>
        {/* Launcher options */}
        <div className='option btn-strd'>
            Options
        </div>
    </div>
  )
}

export default gameList