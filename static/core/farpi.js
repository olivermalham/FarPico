let farPiRoot = null;
let farPiControls = [];

class FarPi extends HTMLElement {

    // State attribute is populated by JSON decoding server response
    state = {}

    // Web-socket connection to the server
    socket = undefined

    // Send a message back to the server
    sendMsg() {
        this.socket.send(document.getElementById('msg').value);
    }

    action(target, args) {
        this.socket.send("{\"action\":\"" + target + "\", \"parameters\":{" + args + "}}");
    }

    // Opens a websocket connection to the server, sets up the message handler and finds and configures all
    // FarPi child elements in the DOM
    // TODO! UPDATE THIS TO SUPPORT HTTP AS WELL AS WEBSOCKETS!!! Do that in the main FarPi-Server codebase
    connectedCallback() {
        setTimeout(() => {

            // Assume we only have one farpi-root element
            farPiRoot = this;

            let address = this.getAttribute("server");

            // If no FarPi server specified, default to same as the webserver
            if(!address){
                address = `ws://${window.location.hostname}:8888/farpi`;
                console.log("Defaulting FarPi Address to " + address);
            }
            //
            // this.socket = new WebSocket(address);
            // console.log("FarPi connected to " + address);
            //
            // // Get all elements with the _farPiComponent class, automatically added by FarPiElement
            // farPiControls = this.getElementsByClassName("_farPiComponent");
            //
            // // Handler for incoming state updates
            // this.socket.onmessage = (e) => {
            //     let state = JSON.parse(e.data);
            //     for (let i = 0; i < farPiControls.length; i++) {
            //         farPiControls[i].farPiUpdate(state);
            //     }
            // }
            //
            // // Handler for the websocket closing. Just kills the heartbeat animations
            // this.socket.onclose = (e) => {
            //     let heartbeat = document.getElementsByTagName("farpi-heartbeat")[0];
            //     heartbeat.disconnected();
            // }
            //
            // // Switch to fullscreen if the parameter is set
            // // FIXME: This seems to be blocked by Chrome
            // if(this.getAttribute("fullscreen") != null){
            //     this.fullscreen();
            // }
        });
    }

    fullscreen(){
        document.documentElement
            .requestFullscreen({ navigationUI: "show" })
            .then(() => {})
            .catch((err) => {
                alert(
                  `An error occurred while trying to switch into fullscreen mode: ${err.message} (${err.name})`
                );
            });
    }
}
customElements.define('farpi-root', FarPi);

class FarPiElement extends HTMLElement {

    // Base class for FarPi elements.
    connectedCallback() {
        // Add classname to make it easy to find all FarPi components
        this.className = this.className + " _farPiComponent";
        this.source = this.getAttribute("source");
        setTimeout(() => this.setup());
    }

    setup() {
        // Setup code run once the DOM is fully constructed
    }

    farPiUpdate(newValue) {
        // Called every time we receive a state update from the server
    }

    action(action, args){
        // Utility function for RPC calls back to the server
        farPiRoot.action(this.source + "." + action, args);
    }
}


class FarPiHeartBeat extends FarPiElement {
    // Simple active connection indicator
    setup() {
        this.classList.add("pr-3");
        this.label = this.innerText;
        this.innerHTML =
            `<div class="flex items-center justify-center text-neutral border border-2 border-neutral p-1 rounded-xl w-12 h-12 items-center">
                <div class="text-neutral inline-block h-6 w-6 animate-spin rounded-full border-4 border-solid border-current border-r-transparent align-[-0.125em] motion-reduce:animate-[spin_1.5s_linear_infinite]" role="status">
                    
                </div>
            </div>`
    }

    disconnected() {
        let spinner = this.getElementsByClassName("animate-spin")[0];
        if(spinner){
            spinner.classList.remove("animate-spin")
        }
        this.innerHTML = `
            <div class="flex items-center justify-center text-neutral border border-2 border-neutral p-1 rounded-xl w-12 h-12 items-center text-5xl">
                    &CircleTimes;
            </div>`
    }
}
customElements.define('farpi-heartbeat', FarPiHeartBeat);


class FarPiPanel extends FarPiElement {
    // Very basic custom component to create a control panel - just a DaisyUI card really

    setup() {
        this.classList.add("card", "card-bordered", "border-panel", "shadow-xl", "bg-neutral", "text-neutral-content", "backdrop-blur", "bg-white/10");
        this.innerHTML = `<div class="card-body">${this.innerHTML}</div>`;
    }

}
customElements.define('farpi-panel', FarPiPanel);
