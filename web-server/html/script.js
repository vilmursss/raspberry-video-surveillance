document.addEventListener('DOMContentLoaded', function() {
    const streamButton = document.getElementById('streamButton');
    const videoContainer = document.getElementById('videoContainer');
    const videoPlayer = document.getElementById('videoPlayer');
    const loadingStatus = document.getElementById('loadingStatus');
    const errorStatus = document.getElementById('errorStatus');

    let isStreaming = false;
    let hls = null;

    streamButton.addEventListener('click', function() {
        if (isStreaming) {
            stopStreaming();
            streamButton.textContent = 'Start Stream';
            videoContainer.style.display = 'none';
        } else {
            startStreaming();
        }
    });

    function startStreaming() {        
        console.log("Starting streaming...");

        videoContainer.style.display = 'block';
        loadingStatus.textContent = "Starting camera stream...";
        loadingStatus.style.display = 'block';
        errorStatus.style.display = 'none';
        videoPlayer.style.display = 'none';

        // First, start the stream on the server
        fetch('/start-stream', {
            method: 'GET',
            headers: {
                'Cache-Control': 'no-cache'
            },
            timeout: 15000
        })
        .then(response => {
            if (!response.ok) {
                throw new Error(`Failed to start stream, status: ${response.status}`);
            }
            return response.json();
        })
        .then(data => {            
            loadingStatus.textContent = "Initializing video stream...";
            setTimeout(() => {
                initializeHlsPlayer();
            }, 5000);
            
            streamButton.textContent = 'Stop Stream';
            isStreaming = true;
        })
        .catch(error => {
            errorStatus.textContent = "Error starting stream: " + error.message;
            errorStatus.style.display = 'block';
            loadingStatus.style.display = 'none';
        });
    }

    function initializeHlsPlayer() {
        if (hls) {
            hls.destroy();
            hls = null;
        }

        if (!Hls.isSupported()) {
            errorStatus.textContent = "HLS is not supported by your browser";
            errorStatus.style.display = 'block';
            loadingStatus.style.display = 'none';
            return;
        }   

        hls = new Hls({
            enableWorker: true,
            lowLatencyMode: false,
            backBufferLength: 30,
            liveSyncDuration: 6,
            liveMaxLatencyDuration: 10,
            liveDurationInfinity: true,

            xhrSetup: function(xhr, url) {
                // Extract segment number from URL
                const segmentMatch = url.match(/segment_(\d+)\.ts/);
                if (segmentMatch) {
                    if (url.includes('http://')) {
                        const segmentFile = 'segment_' + segmentMatch[1] + '.ts';
                        const newUrl = '/' + segmentFile;
                        xhr.open('GET', newUrl, true);
                    }
                    else if (!url.startsWith('/')) {
                        const newUrl = '/' + url;
                        xhr.open('GET', newUrl, true);
                    }
                }
            }
        });

        // Add a timestamp to avoid caching
        const timestamp = new Date().getTime();
        const streamUrl = `/stream.m3u8?t=${timestamp}`;

        // Add event listeners before loading the source
        hls.on(Hls.Events.MANIFEST_PARSED, function() {
            videoPlayer.play()
                .then(() => {
                    loadingStatus.style.display = 'none';
                    videoPlayer.style.display = 'block';
                })
                .catch(e => {
                    errorStatus.textContent = "Error starting playback. Please try again.";
                    errorStatus.style.display = 'block';
                    loadingStatus.style.display = 'none';
                });
        });
        
        hls.loadSource(streamUrl);
        hls.attachMedia(videoPlayer);
    }
    function stopStreaming() {
        console.log("Stopping streaming...");
        
        if (hls) {
            hls.destroy();
            hls = null;
        }
        
        // Stop video playback
        videoPlayer.pause();
        videoPlayer.src = '';
        videoPlayer.load();
        
        // Tell server to stop the stream
        fetch('/stop-stream')
            .then(response => response.json())
            .then(data => {
                console.log("Stream stopped on server:", data);
            })
            .catch(error => {
                console.error("Error stopping stream:", error);
            });
        
        isStreaming = false;
    }
}); 