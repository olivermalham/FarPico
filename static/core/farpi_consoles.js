class FarPiMessageBox extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.label = this.innerText;
        this.innerHTML =
            `<textarea readonly class="textarea textarea-bordered MessageBox w-full h-[16rem]"></textarea>`
        this.onclick = this.onclick_handler
        console.log('FarPiLED added to page - ' + this.source);
    }

    farPiUpdate(newValue) {
        let messageBox = this.getElementsByClassName("MessageBox")[0];

        if(newValue["message"]["text"].length > 0){
            messageBox.value = messageBox.value + newValue["message"]["text"] + "\n";
            messageBox.scrollTop = messageBox.scrollHeight;
        }
    }
}

class FarPiConsole extends FarPiElement {
    setup() {
        this.source = this.getAttribute("source");
        this.buffer = ""
        this.commandLine = ""
        this.echo = (this.getAttribute("echo") != null);
        this.classList.add("h-full");

        this.innerHTML =
            `<div class="form-control h-full">
                <label class="input-group input-group-vertical h-full">
                    <textarea readonly class="textarea textarea-bordered MessageBox w-full h-full bg-black"></textarea>
                    <input type="text" placeholder="Enter command...." class="CommandLine input input-bordered" />
                </label>
            </div>`
        console.log('FarPiConsole added to page');
        this.onkeydown = this.onenter_handler;
    }

    farPiUpdate(newValue) {
        let messageBox = this.getElementsByClassName("MessageBox")[0];

        if(newValue["message"]["text"].length > 0){
            messageBox.value = messageBox.value + newValue["message"]["text"] + "\n";
            messageBox.scrollTop = messageBox.scrollHeight;
        }

        if (newValue.message) {
            this.buffer = this.buffer + newValue.message.text + "\n";
            messageBox.value = this.buffer;
            messageBox.scrollTop = messageBox.scrollHeight;
        }
        if (newValue.error) {
            this.buffer = this.buffer + "ERROR > " + newValue.error.text + "\n";
            messageBox.value = this.buffer;
            messageBox.scrollTop = messageBox.scrollHeight;
        }
    }

    onenter_handler(event){
        if(event.key === "Enter") {
            let commandLine = this.getElementsByClassName("CommandLine")[0];
            let messageBox = this.getElementsByClassName("MessageBox")[0];
            this.action("action_command", `"command": "${commandLine.value}"`);

            if(this.echo) {
                this.buffer = this.buffer + "SEND > " + commandLine.value + "\n";
                messageBox.value = this.buffer;
                messageBox.scrollTop = messageBox.scrollHeight;
            }

            commandLine.value = "";
        }
    }
}


customElements.define('farpi-message', FarPiMessageBox);
customElements.define('farpi-console', FarPiConsole);