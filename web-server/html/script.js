document.addEventListener('DOMContentLoaded', function() {
    const streamButton = document.getElementById('streamButton');
    const videoContainer = document.getElementById('videoContainer');
    const streamImage = document.getElementById('streamImage');
    const loadingStatus = document.getElementById('loadingStatus');
    const errorStatus = document.getElementById('errorStatus');

    let isStreaming = false;
    let streamInterval = null;

    streamButton.addEventListener('click', function() {
        if (isStreaming) {
            stopStreaming();
            streamButton.textContent = 'Watch Snapshot Stream';
            videoContainer.style.display = 'none';
        } else {
            startStreaming();
            streamButton.textContent = 'Stop Snapshot Stream';
        }
    });

    function startStreaming() {        
        console.log("Starting streaming...");

        videoContainer.style.display = 'block';
        loadingStatus.style.display = 'block';
        errorStatus.style.display = 'none';
        streamImage.style.display = 'none';

        isStreaming = true;

        requestFrame();
    }

    function stopStreaming() {
        console.log("Stopping streaming...");
        isStreaming = false;

        if (streamInterval) {
            clearTimeout(streamInterval);
            streamInterval = null;
        }
    }

    function requestFrame() {
        if (!isStreaming) {
            console.log("Not streaming, skipping frame request");
            return;
        }

        loadFrame().then(success => {
            if (success && isStreaming) {
                requestFrame();
            } else if (isStreaming) {
                console.log("Frame failed to load, retrying in 1 second...");
                streamInterval = setTimeout(requestFrame, 1000);
            }
        }).catch(error => {
            console.error("Error loading frame:", error);
            if (isStreaming) {
                console.log("Error caught, retrying in 1 second...");
                streamInterval = setTimeout(requestFrame, 1000);
            }
        });
    }

    function loadFrame() {
        return new Promise((resolve, reject) => {
            const timestamp = new Date().getTime();
            const imageUrl = `/snapshot?t=${timestamp}`;

            fetch(imageUrl, { 
                method: 'GET',
                cache: 'no-store',
                headers: {
                    'Cache-Control': 'no-cache, no-store, must-revalidate',
                    'Pragma': 'no-cache'
                }
            })
            .then(response => {
                if (!response.ok) {
                    throw new Error(`HTTP error! status: ${response.status}`);
                }
                return response.blob();
            })
            .then(blob => {
                const url = URL.createObjectURL(blob);

                streamImage.onload = function() {
                    URL.revokeObjectURL(url);
                    loadingStatus.style.display = 'none';
                    errorStatus.style.display = 'none';
                    streamImage.style.display = 'block';
                    resolve(true);
                };

                streamImage.onerror = function() {
                    URL.revokeObjectURL(url);
                    loadingStatus.style.display = 'none';
                    errorStatus.style.display = 'block';
                    
                    resolve(false);
                };

                streamImage.src = url;
            })
            .catch(error => {
                console.error("Fetch error:", error);
                loadingStatus.style.display = 'none';
                errorStatus.style.display = 'block';
                errorStatus.textContent = `Error: ${error.message}`;

                resolve(false);
            });
        });
    }
}); 