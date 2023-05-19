
let pc = null;

class FarPiCameraWebRTC extends FarPiElement {
    /*
        Live video feed from the server, using aiortc and WebRTC
     */
    setup() {
        this.soucre = this.getAttribute("source");
        this.classList.add("w-full", "h-full");
        this.innerHTML =
            `<div id="media" class="w-full h-full">
                <video id="video" autoplay muted class="rounded-lg w-full h-full"></video>
            </div>`
        this.start();
        console.log('FarPiCameraWebRTC added to page - ' + this.source);
    }

    negotiate() {
        pc.addTransceiver('video', {direction: 'recvonly'});
        // pc.addTransceiver('audio', {direction: 'recvonly'});
        return pc.createOffer().then(function (offer) {
            return pc.setLocalDescription(offer);
        }).then(function () {
            // wait for ICE gathering to complete
            return new Promise(function (resolve) {
                if (pc.iceGatheringState === 'complete') {
                    resolve();
                } else {
                    function checkState() {
                        if (pc.iceGatheringState === 'complete') {
                            pc.removeEventListener('icegatheringstatechange', checkState);
                            resolve();
                        }
                    }
                    pc.addEventListener('icegatheringstatechange', checkState);
                }
            });
        }).then(function () {
            let offer = pc.localDescription;
            return fetch(`http://192.168.0.44:8080/offer`, {  // TODO: Need to fix this static URL!!!
                body: JSON.stringify({
                    sdp: offer.sdp,
                    type: offer.type,
                }),
                headers: {
                    'Content-Type': 'application/json'
                },
                method: 'POST'
            });
        }).then(function (response) {
            return response.json();
        }).then(function (answer) {
            return pc.setRemoteDescription(answer);
        }).catch(function (e) {
            alert(e);
        });
    }

    start() {
        let config = {
            sdpSemantics: 'unified-plan'
        };

        pc = new RTCPeerConnection(config);

        // connect audio / video
        pc.addEventListener('track', function (evt) {
            if (evt.track.kind == 'video') {
                document.getElementById('video').srcObject = evt.streams[0];
            } else {
                document.getElementById('audio').srcObject = evt.streams[0];
            }
        });

        this.negotiate();
    }

    stop() {
        document.getElementById('stop').style.display = 'none';

        // close peer connection
        setTimeout(function () {
            pc.close();
        }, 500);
    }
}
customElements.define('farpi-camera-webrtc', FarPiCameraWebRTC);

class FarPiCameraMultipart extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");

        // If no FarPi server specified, default to same as the webserver
        if(!this.source){
            this.source = `http://${window.location.hostname}:8889`;
            console.log("Defaulting FarPiCameraMultipart Address to " + this.source);
        }

        this.classList.add("w-full", "h-full");
        this.innerHTML =
            `<img src="${this.source}/video_feed" alt="FarPi multipart image video feed from ${this.source}" class="rounded-lg" />`
        console.log('FarPiCameraMultipart added to page - ' + this.source);
    }
}
customElements.define('farpi-camera-multipart', FarPiCameraMultipart);
