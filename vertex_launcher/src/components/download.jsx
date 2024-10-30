import React from 'react'

const isDownloaded = true;

function download() {
    if(isDownloaded){
        return (
            <div>
                Start
            </div>
          )
    }
    return (
        <div>
            Download
        </div>
      )
  
}

export default download